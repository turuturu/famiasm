
    .inesprg 1
    .ineschr 0
    .inesmir 1
    .inesmap 0

    .bank 0
    .org $C000

START:
    LDX #$00
LOOP:
    INX
    CPX #$10
    BNE LOOP
    
    LDA #$00
    CMP #$00
    BEQ EQUAL
    JMP NOT_EQUAL
    
EQUAL:
    LDA #$01
    JMP END
    
NOT_EQUAL:
    LDA #$02
    
END:
    JMP START

    .org $FFFA
    .dw 0
    .dw START
    .dw 0
