use std::fs;
use std::process::Command;

#[cfg(test)]
mod tests {
    use super::*;

    fn compile_with_famiasm(input: &str, output: &str) -> Result<(), String> {
        let status = Command::new("cargo")
            .args(&["run", "--", input])
            .status()
            .map_err(|e| format!("Failed to run famiasm: {}", e))?;

        if !status.success() {
            return Err("famiasm compilation failed".to_string());
        }

        // Move the output file to the desired location
        let default_output = input.replace(".asm", ".nes");
        if default_output != output {
            fs::rename(&default_output, output)
                .map_err(|e| format!("Failed to move output file: {}", e))?;
        }

        Ok(())
    }

    fn compile_with_nesasm(input: &str, output: &str) -> Result<(), String> {
        let status = Command::new("nesasm")
            .args(&[input, "-o", output])
            .status()
            .map_err(|e| format!("Failed to run nesasm: {}", e))?;

        if !status.success() {
            return Err("nesasm compilation failed".to_string());
        }

        Ok(())
    }

    fn compare_binaries(file1: &str, file2: &str) -> Result<(), String> {
        let data1 = fs::read(file1)
            .map_err(|e| format!("Failed to read {}: {}", file1, e))?;
        let data2 = fs::read(file2)
            .map_err(|e| format!("Failed to read {}: {}", file2, e))?;

        if data1.len() != data2.len() {
            return Err(format!("File sizes differ: {} bytes vs {} bytes", data1.len(), data2.len()));
        }

        for (i, (byte1, byte2)) in data1.iter().zip(data2.iter()).enumerate() {
            if byte1 != byte2 {
                return Err(format!("Files differ at offset 0x{:04X}: 0x{:02X} vs 0x{:02X}", i, byte1, byte2));
            }
        }

        Ok(())
    }

    fn run_comparison_test(test_name: &str, asm_content: &str) {
        let fixture_path = format!("tests/fixtures/{}.asm", test_name);
        let famiasm_output = format!("tests/expected/{}_famiasm.nes", test_name);
        let nesasm_output = format!("tests/expected/{}_nesasm.nes", test_name);

        // Write the test assembly file
        fs::write(&fixture_path, asm_content).expect("Failed to write test file");

        // Compile with both assemblers
        if let Err(e) = compile_with_famiasm(&fixture_path, &famiasm_output) {
            panic!("famiasm compilation failed: {}", e);
        }

        if let Err(e) = compile_with_nesasm(&fixture_path, &nesasm_output) {
            // If nesasm is not available, skip the comparison
            eprintln!("Warning: nesasm not available, skipping comparison: {}", e);
            return;
        }

        // Compare the outputs
        if let Err(e) = compare_binaries(&famiasm_output, &nesasm_output) {
            panic!("Binary comparison failed for {}: {}", test_name, e);
        }
    }

    #[test]
    fn test_basic_instructions() {
        let asm = r#"
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
"#;
        run_comparison_test("basic_instructions", asm);
    }

    #[test]
    fn test_addressing_modes() {
        let asm = r#"
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
"#;
        run_comparison_test("addressing_modes", asm);
    }

    #[test]
    fn test_data_directives() {
        let asm = r#"
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
"#;
        run_comparison_test("data_directives", asm);
    }

    #[test]
    fn test_labels_and_branches() {
        let asm = r#"
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
"#;
        run_comparison_test("labels_and_branches", asm);
    }

    #[test]
    fn test_all_opcodes() {
        let asm = r#"
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
"#;
        run_comparison_test("all_opcodes", asm);
    }

    #[test]
    fn test_org_directive() {
        let asm = r#"
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
"#;
        run_comparison_test("org_directive", asm);
    }
}