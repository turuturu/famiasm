
    .inesprg 1
    .ineschr 0
    .inesmir 1
    .inesmap 0

    .bank 0
    .org $C000

START:
    LDA #$00        ; Load immediate
    STA $2000       ; Store absolute
    LDX #$FF        ; Load X immediate
    STX $2001       ; Store X absolute
    LDY #$10        ; Load Y immediate
    STY $2002       ; Store Y absolute
    NOP             ; No operation
    JMP START       ; Jump absolute

    .org $FFFA
    .dw 0           ; NMI vector
    .dw START       ; Reset vector
    .dw 0           ; IRQ vector
