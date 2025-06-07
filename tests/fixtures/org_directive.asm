
    .inesprg 1
    .ineschr 0
    .inesmir 1
    .inesmap 0

    .bank 0
    .org $8000
    .db $11, $22, $33
    
    .org $9000
    .db $44, $55, $66
    
    .org $C000
START:
    LDA #$00
    JMP START

    .org $FFFA
    .dw 0
    .dw START
    .dw 0
