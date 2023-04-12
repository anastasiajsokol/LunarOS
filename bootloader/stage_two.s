org 0x0
bits 16

_main:
    cli ; clear interrupts
    
    ; ensure cs = ds
    push cs
    pop ds

    ; print stage two message
    mov si, msg
    lodsb
    .print:
        mov ah, 0x0e
        int 0x10
        lodsb
        or al, al
        jnz .print
    
    cli
    hlt

msg db "Lunar OS - Stage Two Bootloader", 0
