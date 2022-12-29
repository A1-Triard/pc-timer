use core::arch::asm;
use core::mem::{replace, size_of};
use core::ptr::{self, null};
use pc_ints::*;

const IRQ_0_HANDLER: &[u8] = &[
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // ticks (0000h): dq 0
    0x00, 0x00, // low_level_ticks_per_tick (0008h): dw 0
    0x00, 0x00, // low_level_ticks_mod_10000h (000Ah): dw 0
    0x00, 0x00, 0x00, 0x00, // bios_handler (000Ch): dd 0
    0x00, // init (0010h): db 0
    0x9C, //handler_entry (0011h): pushf
    0x1E, // push ds
    0x53, // push bx
    0x50, // push ax
    0x8C, 0xC8, // mov ax, cs
    0x8E, 0xD8, // mov ds, ax
    0xC6, 0x06, 0x10, 0x00, 0x01, // mov byte [init], 1
    0xBB, 0x00, 0x00, // mov bx, ticks
    0x83, 0x07, 0x01, // add word [bx], 1
    0x83, 0x57, 0x02, 0x00, // adc word [bx+2], 0
    0x83, 0x57, 0x04, 0x00, // adc word [bx+4], 0
    0x83, 0x57, 0x06, 0x00, // adc word [bx+6], 0
    0xA1, 0x08, 0x00, // mov ax, [low_level_ticks_per_tick]
    0x01, 0x06, 0x0A, 0x00, // add [low_level_ticks_mod_10000h], ax
    0x73, 0x08, // jnc skip_bios_handler
    0x58, // pop ax
    0x5B, // pop bx
    0x1F, // pop ds
    0x9D, // popf
    0xFF, 0x2E, 0x0C, 0x00, // jmp far [bios_handler]
    0xB0, 0x20, // skip_bios_handler: mov al, 0x20
    0xE6, 0x20, // out 0x20, al
    0x58, // pop ax
    0x5B, // pop bx
    0x1F, // pop ds
    0x9D, // popf
    0xCF, // iret
];

static mut TICKS: *const u64 = null();

pub unsafe fn init(frequency: u16) {
    let low_level_ticks_per_tick = (0x1234DDu32 / frequency as u32).try_into().expect("frequency >= 19");
    let irq_0_handler_segment = int_31h_ax_0100h_rm_alloc((IRQ_0_HANDLER.len().checked_add(15).unwrap() / 16).try_into().unwrap())
        .expect("cannot allocate real-mode memory for timer");
    let irq_0_handler_segment = irq_0_handler_segment.ax_segment;
    assert!(size_of::<usize>() == size_of::<u32>());
    let irq_0_handler_addr = ((irq_0_handler_segment as u32) << 4) as usize;
    ptr::copy_nonoverlapping(IRQ_0_HANDLER.as_ptr(), irq_0_handler_addr as *mut u8, IRQ_0_HANDLER.len());
    ptr::write_unaligned((irq_0_handler_addr + 0x0008) as *mut u16, low_level_ticks_per_tick);
    asm! { "cli" }
    let bios_handler = int_31h_ax_0200h_get_rm_int(8);
    ptr::write_unaligned((irq_0_handler_addr + 0x000C) as *mut u16, bios_handler.dx_offset);
    ptr::write_unaligned((irq_0_handler_addr + 0x000C + 2) as *mut u16, bios_handler.cx_segment);
    int_31h_ax_0201h_set_rm_int(8, irq_0_handler_segment, 0x0011);
    asm! {
        "out 0x43, al",
        in ("ax") 0x34u16
    }
    asm! {
        "out 0x40, al",
        in ("ax") (low_level_ticks_per_tick & 0xFF) as u8 as u16
    }
    asm! {
        "out 0x40, al",
        in ("ax") (low_level_ticks_per_tick >> 8) as u8 as u16
    }
    asm! { "sti" }
    loop {
        if ptr::read_volatile((irq_0_handler_addr + 0x0010) as *const u8) != 0 { break; }
    }
    TICKS = irq_0_handler_addr as *const u64;
}

pub unsafe fn ticks() -> u64 {
    asm! { "cli" }
    let ticks = *TICKS;
    asm! { "sti" }
    ticks
}

pub unsafe fn done() {
    asm! { "cli" }
    let irq_0_handler_addr = replace(&mut TICKS, null()) as usize;
    let bios_handler_offset = ptr::read_unaligned((irq_0_handler_addr + 0x000C) as *mut u16);
    let bios_handler_segment = ptr::read_unaligned((irq_0_handler_addr + 0x000C + 2) as *mut u16);
    int_31h_ax_0201h_set_rm_int(8, bios_handler_segment, bios_handler_offset);
    asm! {
        "out 0x43, al",
        in ("ax") 0x34u16
    }
    asm! {
        "out 0x40, al",
        in ("ax") 0u16,
    }
    asm! {
        "out 0x40, al",
        in ("ax") 0u16
    }
    asm! { "sti" }
}
