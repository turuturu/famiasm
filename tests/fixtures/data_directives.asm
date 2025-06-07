
    .inesprg 1
    .ineschr 0
    .inesmir 1
    .inesmap 0

    .bank 0
    .org $C000

DATA:
    .db $00, $01, $02, $03, $04, $05
    .db $FF, $FE, $FD, $FC, $FB, $FA
    
    .dw $1234, $5678, $9ABC, $DEF0
    .dw DATA, START

START:
    LDA DATA
    LDX #$01
    LDY #$02
    JMP START

    .org $FFFA
    .dw 0
    .dw START
    .dw 0
