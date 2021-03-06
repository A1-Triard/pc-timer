use core::arch::asm;
use core::mem::size_of;
use pc_ints::*;

struct Data {
    ints: u64,
    ticks_mod_10000: u16,
    ticks_per_int: u16,
    bios_int_handler: u32,
}

static mut DATA: Data = Data {
    ints: 0,
    ticks_mod_10000: 0,
    ticks_per_int: 0,
    bios_int_handler: 0,
};

#[naked]
unsafe extern "C" fn int_8_handler_entry() {
    asm! {
        "pushad",
        "sub esp, 4",
        "push esp",
        "call {int_8_handler}",
        "add esp, 4",
        "pop edx",
        "xor ebx, ebx",
        "or eax, ebx",
        "jz 1f",
        "call edx",
        "jmp 2f",
        "1: mov al, 0x20",
        "out 0x20, al",
        "2: popad",
        "iret",
        int_8_handler = sym int_8_handler,
        options(noreturn)
    }
}

unsafe extern "C" fn int_8_handler(bios_int_handler: *mut u32) -> u8 {
    DATA.ints = DATA.ints.wrapping_add(1);
    DATA.ticks_mod_10000 += DATA.ticks_per_int;
    *bios_int_handler = DATA.bios_int_handler;
    if DATA.ticks_mod_10000 >= 10000 {
        DATA.ticks_mod_10000 -= 10000;
        debug_assert!(DATA.ticks_mod_10000 < 10000);
        1
    } else {
        0
    }
}

fn p32<T>(ptr: *const T) -> u32 {
    assert!(size_of::<usize>() == size_of::<u32>());
    ptr as usize as u32
}

pub unsafe fn init(frequency: u16) {
    let ticks_per_int = (0x1234DDu32 / frequency as u32).try_into().ok().filter(|&x| x < 10000)
        .expect("frequency >= 120");
    asm! { "cli" }
    DATA.ints = 0;
    DATA.ticks_mod_10000 = 0;
    DATA.bios_int_handler = int_21h_ah_35h_get_int(8).ebx_int_handler;
    DATA.ticks_per_int = ticks_per_int;
    int_21h_ah_25h_set_int(8, p32(int_8_handler_entry as *const u8));
    asm! {
        "out 0x43, al",
        in ("eax") 0x34u32
    } asm! { "out 0x40, al",
        in ("eax") ticks_per_int as u8 as u32,
    }
    asm! {
        "out 0x40, al",
        in ("eax") (ticks_per_int >> 8) as u8 as u32
    }
    asm! { "sti" }
}

pub unsafe fn done() {
    asm! { "cli" }
    int_21h_ah_25h_set_int(8, DATA.bios_int_handler);
    asm! {
        "out 0x43, al",
        in ("eax") 0x34u32
    }
    asm! {
        "out 0x40, al",
        in ("eax") 0u32,
    }
    asm! {
        "out 0x40, al",
        in ("eax") 0u32
    }
    asm! { "sti" }
}

pub unsafe fn ticks() -> u64 {
    asm! { "cli" }
    let ticks = DATA.ints;
    asm! { "sti" }
    ticks
}
