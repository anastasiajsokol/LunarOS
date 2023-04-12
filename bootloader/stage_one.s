org 0x7c00
bits 16

_start:
    ; clear sectors
    xor	ax, ax
    mov ds, ax
    mov es, ax

    mov [drive], dl

    ; print message
    mov si, msg
    call print_string

    ; reset drive (note that dl should hold the id of the drive the bootloader was loaded from)
    call reset_drive

    ; load second stage from memory
    mov ax, 0x1000
    call read_stage

    ; get the number of kb of available ram
    xor ax, ax
    int 0x12    

    ; jump to loaded second stage
    jmp 0x1000:00

    ; halt just in case
    cli
    hlt

print_string: ; (si data: string)
    push ax
    lodsb
    .print:
        mov ah, 0x0e
        int 0x10
        
        lodsb
        or al, al
        jnz .print

    pop ax
    ret

reset_drive: ; ()
    push ax
    .reset:
        mov ah, 0
        mov dl, [drive]
        int 0x13
        jc .reset
    pop ax
    ret

read_stage: ; (ax location: address)
    ; load new segment offset from ax
    mov ds, ax
    mov es, ax

    .read:
        mov ah, 0x02 ; function 2
        mov al, 0x01 ; read 1 sector
        mov ch, 0x01 ; track 1
        mov cl, 0x02 ; second sector
        mov dh, 0x00 ; head zero
        
        mov dl, [drive]

        int 0x13
        jc .read

    ret

msg db "Lunar OS - Stage One Bootloader", 0
drive db 0

times 510 - ($-$$) db 0

dw 0xaa55