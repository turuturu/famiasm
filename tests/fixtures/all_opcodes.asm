
    .inesprg 1
    .ineschr 0
    .inesmir 1
    .inesmap 0

    .bank 0
    .org $C000

START:
    ; ADC - Add with Carry
    ADC #$01
    ADC $10
    ADC $10,X
    ADC $1234
    ADC $1234,X
    ADC $1234,Y
    ADC ($10,X)
    ADC ($10),Y
    
    ; AND - Logical AND
    AND #$FF
    AND $10
    AND $10,X
    AND $1234
    AND $1234,X
    AND $1234,Y
    AND ($10,X)
    AND ($10),Y
    
    ; ASL - Arithmetic Shift Left
    ASL A
    ASL $10
    ASL $10,X
    ASL $1234
    ASL $1234,X
    
    ; Branch Instructions
    BCC NEXT1
NEXT1:
    BCS NEXT2
NEXT2:
    BEQ NEXT3
NEXT3:
    BMI NEXT4
NEXT4:
    BNE NEXT5
NEXT5:
    BPL NEXT6
NEXT6:
    BVC NEXT7
NEXT7:
    BVS NEXT8
NEXT8:
    
    ; BIT - Bit Test
    BIT $10
    BIT $1234
    
    ; BRK - Force Interrupt
    BRK
    
    ; Clear/Set Flags
    CLC
    CLD
    CLI
    CLV
    SEC
    SED
    SEI
    
    ; CMP - Compare
    CMP #$10
    CMP $10
    CMP $10,X
    CMP $1234
    CMP $1234,X
    CMP $1234,Y
    CMP ($10,X)
    CMP ($10),Y
    
    ; CPX - Compare X Register
    CPX #$10
    CPX $10
    CPX $1234
    
    ; CPY - Compare Y Register
    CPY #$10
    CPY $10
    CPY $1234
    
    ; DEC - Decrement Memory
    DEC $10
    DEC $10,X
    DEC $1234
    DEC $1234,X
    
    ; DEX/DEY - Decrement Registers
    DEX
    DEY
    
    ; EOR - Exclusive OR
    EOR #$FF
    EOR $10
    EOR $10,X
    EOR $1234
    EOR $1234,X
    EOR $1234,Y
    EOR ($10,X)
    EOR ($10),Y
    
    ; INC - Increment Memory
    INC $10
    INC $10,X
    INC $1234
    INC $1234,X
    
    ; INX/INY - Increment Registers
    INX
    INY
    
    ; JMP - Jump
    JMP $1234
    JMP ($1234)
    
    ; JSR - Jump to Subroutine
    JSR $1234
    
    ; LDA - Load Accumulator
    LDA #$42
    LDA $10
    LDA $10,X
    LDA $1234
    LDA $1234,X
    LDA $1234,Y
    LDA ($10,X)
    LDA ($10),Y
    
    ; LDX - Load X Register
    LDX #$42
    LDX $10
    LDX $10,Y
    LDX $1234
    LDX $1234,Y
    
    ; LDY - Load Y Register
    LDY #$42
    LDY $10
    LDY $10,X
    LDY $1234
    LDY $1234,X
    
    ; LSR - Logical Shift Right
    LSR A
    LSR $10
    LSR $10,X
    LSR $1234
    LSR $1234,X
    
    ; NOP - No Operation
    NOP
    
    ; ORA - Logical Inclusive OR
    ORA #$FF
    ORA $10
    ORA $10,X
    ORA $1234
    ORA $1234,X
    ORA $1234,Y
    ORA ($10,X)
    ORA ($10),Y
    
    ; Push/Pull
    PHA
    PHP
    PLA
    PLP
    
    ; ROL - Rotate Left
    ROL A
    ROL $10
    ROL $10,X
    ROL $1234
    ROL $1234,X
    
    ; ROR - Rotate Right
    ROR A
    ROR $10
    ROR $10,X
    ROR $1234
    ROR $1234,X
    
    ; RTI - Return from Interrupt
    RTI
    
    ; RTS - Return from Subroutine
    RTS
    
    ; SBC - Subtract with Carry
    SBC #$01
    SBC $10
    SBC $10,X
    SBC $1234
    SBC $1234,X
    SBC $1234,Y
    SBC ($10,X)
    SBC ($10),Y
    
    ; STA - Store Accumulator
    STA $10
    STA $10,X
    STA $1234
    STA $1234,X
    STA $1234,Y
    STA ($10,X)
    STA ($10),Y
    
    ; STX - Store X Register
    STX $10
    STX $10,Y
    STX $1234
    
    ; STY - Store Y Register
    STY $10
    STY $10,X
    STY $1234
    
    ; Transfer Instructions
    TAX
    TAY
    TSX
    TXA
    TXS
    TYA

    .org $FFFA
    .dw 0
    .dw START
    .dw 0
