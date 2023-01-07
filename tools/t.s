bits 16
cpu 386
org 0x100

freq equ 120 ; ticks per second

    mov ax, cs
    mov ds, ax
    cli
    mov word [low_level_ticks_per_tick], 0x1234DD / freq
    mov al, 8
    mov ah, 0x35
    int 0x21
    mov [bios_handler], bx
    mov [bios_handler+2], es
    mov ah, 0x25
    mov dx, handler_entry
    int 0x21
    mov al, 0x34
    out 0x43, al
    mov al, (0x1234DD / freq) % 256
    out 0x40, al
    mov al, (0x1234DD / freq) / 256
    out 0x40, al
    sti
wait_init:
    mov al, [init]
    test al, al
    jz wait_init
    jmp timer
prev_ticks:
    dq 0
next_ticks:
    dq 0
ticks_delta:
    dq 0
msg:
    db "000000", 0x0D, 0x0A, "$"
timer:
    cli
    mov ax, [ticks]
    mov [prev_ticks], ax
    mov ax, [ticks+2]
    mov [prev_ticks+2], ax
    mov ax, [ticks+4]
    mov [prev_ticks+4], ax
    mov ax, [ticks+6]
    mov [prev_ticks+6], ax
    sti
timer_loop:
    mov ah, 0x06
    mov dl, 0xFF
    int 0x21
    lahf
    test ah, 0x40
    jnz timer_continue
    mov ah, 0x4C
    int 0x21
    hlt
timer_continue:
    mov cx, 128
timer_nops:
    nop
    loop timer_nops
    cli
    mov ax, [ticks]
    mov [next_ticks], ax
    mov [ticks_delta], ax
    mov ax, [ticks+2]
    mov [next_ticks+2], ax
    mov [ticks_delta+2], ax
    mov ax, [ticks+4]
    mov [next_ticks+4], ax
    mov [ticks_delta+4], ax
    mov ax, [ticks+6]
    mov [next_ticks+6], ax
    mov [ticks_delta+6], ax
    sti
    mov bx, ticks_delta
    mov ax, [prev_ticks]
    sub [bx], ax
    mov ax, [prev_ticks+2]
    sbb [bx+2], ax
    mov ax, [prev_ticks+4]
    sbb [bx+4], ax
    mov ax, [prev_ticks+6]
    sbb [bx+6], ax
    cmp word [bx+6], 0
    jg print
    cmp word [bx+4], 0
    jg print
    cmp word [bx+2], 0
    jg print
    cmp word [bx], freq
    jl timer_loop
print:
    add byte [msg+5], 1
    cmp byte [msg+5], '9'
    jle print_msg
    mov byte [msg+5], '0'
    add byte [msg+4], 1
    cmp byte [msg+4], '9'
    jle print_msg
    mov byte [msg+4], '0'
    add byte [msg+3], 1
    cmp byte [msg+3], '9'
    jle print_msg
    mov byte [msg+3], '0'
    add byte [msg+2], 1
    cmp byte [msg+2], '9'
    jle print_msg
    mov byte [msg+2], '0'
    add byte [msg+1], 1
    cmp byte [msg+1], '9'
    jle print_msg
    mov byte [msg+1], '0'
    add byte [msg], 1
    cmp byte [msg], '9'
    jle print_msg
    mov byte [msg], '0'
print_msg:
    mov dx, msg
    mov ah, 0x09
    int 0x21
    mov ax, [next_ticks]
    mov [prev_ticks], ax
    mov ax, [next_ticks+2]
    mov [prev_ticks+2], ax
    mov ax, [next_ticks+4]
    mov [prev_ticks+4], ax
    mov ax, [next_ticks+6]
    mov [prev_ticks+6], ax
    jmp timer_loop
    db 0
ticks:
    dq 0
low_level_ticks_per_tick:
    dw 0
low_level_ticks_mod_10000h:
    dw 0
init:
    db 0
handler_entry:
    pushf
    push ds
    push bx
    push ax
    mov ax, cs
    mov ds, ax
    mov byte [init], 1
    mov bx, ticks
    add word [bx], 1
    adc word [bx+2], 0
    adc word [bx+4], 0
    adc word [bx+6], 0
    mov ax, [low_level_ticks_per_tick]
    add [low_level_ticks_mod_10000h], ax
    jnc skip_bios_handler
    pop ax
    pop bx
    pop ds
    popf
    db 0xEA
bios_handler:
    dd 0
skip_bios_handler:
    mov al, 0x20
    out 0x20, al
    pop ax
    pop bx
    pop ds
    popf
    iret
