
    .inesprg 1
    .ineschr 0
    .inesmir 1
    .inesmap 0

    .bank 0
    .org $C000

START:
    ; Immediate
    LDA #$42
    
    ; Zero Page
    LDA $00
    STA $01
    
    ; Zero Page,X
    LDA $00,X
    STA $01,X
    
    ; Zero Page,Y
    LDX $00,Y
    STX $01,Y
    
    ; Absolute
    LDA $1234
    STA $5678
    
    ; Absolute,X
    LDA $1234,X
    STA $5678,X
    
    ; Absolute,Y
    LDA $1234,Y
    STA $5678,Y
    
    ; Indirect
    JMP ($1234)
    
    ; Indexed Indirect (zp,X)
    LDA ($40,X)
    STA ($42,X)
    
    ; Indirect Indexed (zp),Y
    LDA ($40),Y
    STA ($42),Y
    
    ; Implied
    INX
    DEY
    CLC
    SEC
    
    ; Relative
    BEQ SKIP
    LDA #$00
SKIP:
    BNE START

    .org $FFFA
    .dw 0
    .dw START
    .dw 0
