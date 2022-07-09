pub unsafe fn init(frequency: u16) {
    let ticks_per_int = (0x1234DDu32 / frequency as u32).try_into().ok().filter(|&x| x < 10000)
        .expect("frequency >= 120");
    asm! { "cli" }
    DATA.ticks = 0;
    DATA.ticks_mod_10000 = 0;
    DATA.bios_int_handler = int_21h_ah_35h_get_int(8).ebx_int_handler;
    DATA.ticks_per_int = ticks_per_int;
    int_21h_ah_25h_set_int(8, p32(int_8_handler_entry as *const u8));
    asm! {
        "out 0x43, al",
        in ("eax") 0x34u32
    }
    asm! {
        "out 0x40, al",
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
    let ticks;
    asm! { "cli" }
    ticks = DATA.ticks;
    asm! { "sti" }
    ticks
}
