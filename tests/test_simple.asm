    .inesprg 1
    .ineschr 0
    .inesmir 1
    .inesmap 0

    .bank 0
    .org $C000

START:
    LDA #$00
    STA $2000
    JMP START

    .org $FFFA
    .dw 0
    .dw START
    .dw 0