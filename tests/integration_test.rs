#[cfg(test)]
mod integration_tests {
    use std::fs;
    use std::process::Command;

    fn ensure_binary_built() {
        let output = Command::new("cargo")
            .args(&["build"])
            .output()
            .expect("Failed to build project");

        if !output.status.success() {
            panic!("Failed to build famiasm");
        }
    }

    fn run_famiasm(input_file: &str) -> Result<Vec<u8>, String> {
        ensure_binary_built();

        let output = Command::new("cargo")
            .args(&["run", "--quiet", "--", input_file])
            .output()
            .map_err(|e| format!("Failed to run famiasm: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "famiasm failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        // Read the output file
        let output_file = input_file.replace(".asm", ".nes");
        fs::read(&output_file).map_err(|e| format!("Failed to read output file: {}", e))
    }

    #[test]
    fn test_minimal_rom() {
        let test_asm = r#"
    .inesprg 1
    .ineschr 0
    .inesmir 1
    .inesmap 0

    .bank 0
    .org $C000

START:
    JMP START

    .org $FFFA
    .dw 0
    .dw START
    .dw 0
"#;
        let test_file = "test_minimal.asm";
        fs::write(test_file, test_asm).expect("Failed to write test file");

        let result = run_famiasm(test_file);
        assert!(result.is_ok(), "Failed to assemble minimal ROM");

        let output = result.unwrap();

        // Check iNES header
        assert_eq!(&output[0..4], b"NES\x1A", "Invalid NES header magic");
        assert_eq!(output[4], 1, "Wrong PRG-ROM count");
        assert_eq!(output[5], 0, "Wrong CHR-ROM count");
        assert_eq!(output[6] & 0x01, 1, "Wrong mirroring flag");
        assert_eq!(output[7] & 0xF0, 0, "Wrong mapper high nibble");

        // Check that the ROM has the correct size
        // Header (16) + PRG-ROM (16384)
        assert_eq!(output.len(), 16 + 16384, "Wrong ROM size");

        // Clean up
        fs::remove_file(test_file).ok();
        fs::remove_file("test_minimal.nes").ok();
    }

    #[test]
    fn test_data_bytes() {
        let test_asm = r#"
    .inesprg 1
    .ineschr 0
    .inesmir 1
    .inesmap 0

    .bank 0
    .org $C000
    .db $01, $02, $03, $04
    .db $AA, $BB, $CC, $DD

START:
    JMP START

    .org $FFFA
    .dw 0
    .dw START
    .dw 0
"#;
        let test_file = "test_data_bytes.asm";
        fs::write(test_file, test_asm).expect("Failed to write test file");

        let result = run_famiasm(test_file);
        assert!(result.is_ok(), "Failed to assemble ROM with data bytes");

        let output = result.unwrap();

        // Check data bytes at the beginning of PRG-ROM
        // PRG-ROM starts at offset 16 (after header)
        assert_eq!(output[16], 0x01);
        assert_eq!(output[17], 0x02);
        assert_eq!(output[18], 0x03);
        assert_eq!(output[19], 0x04);
        assert_eq!(output[20], 0xAA);
        assert_eq!(output[21], 0xBB);
        assert_eq!(output[22], 0xCC);
        assert_eq!(output[23], 0xDD);

        // Clean up
        fs::remove_file(test_file).ok();
        fs::remove_file("test_data_bytes.nes").ok();
    }

    #[test]
    fn test_basic_opcodes() {
        let test_asm = r#"
    .inesprg 1
    .ineschr 0
    .inesmir 1
    .inesmap 0

    .bank 0
    .org $C000

START:
    LDA #$42        ; A9 42
    LDX #$FF        ; A2 FF
    LDY #$00        ; A0 00
    NOP             ; EA
    JMP START       ; 4C 00 C0

    .org $FFFA
    .dw 0
    .dw START
    .dw 0
"#;
        let test_file = "test_opcodes.asm";
        fs::write(test_file, test_asm).expect("Failed to write test file");

        let result = run_famiasm(test_file);
        assert!(result.is_ok(), "Failed to assemble ROM with opcodes");

        let output = result.unwrap();

        // Check opcodes at the beginning of PRG-ROM
        assert_eq!(output[16], 0xA9); // LDA immediate
        assert_eq!(output[17], 0x42); // #$42
        assert_eq!(output[18], 0xA2); // LDX immediate
        assert_eq!(output[19], 0xFF); // #$FF
        assert_eq!(output[20], 0xA0); // LDY immediate
        assert_eq!(output[21], 0x00); // #$00
        assert_eq!(output[22], 0xEA); // NOP
        assert_eq!(output[23], 0x4C); // JMP absolute
        assert_eq!(output[24], 0x00); // Low byte of $C000
        assert_eq!(output[25], 0xC0); // High byte of $C000

        // Clean up
        fs::remove_file(test_file).ok();
        fs::remove_file("test_opcodes.nes").ok();
    }

    #[test]
    fn test_reset_vector() {
        let test_asm = r#"
    .inesprg 1
    .ineschr 0
    .inesmir 1
    .inesmap 0

    .bank 0
    .org $C000

START:
    JMP START

    .org $FFFA
    .dw $1234       ; NMI vector
    .dw $5678       ; Reset vector
    .dw $9ABC       ; IRQ vector
"#;
        let test_file = "test_vectors.asm";
        fs::write(test_file, test_asm).expect("Failed to write test file");

        let result = run_famiasm(test_file);
        assert!(result.is_ok(), "Failed to assemble ROM with vectors");

        let output = result.unwrap();

        // Vectors are at the very end of the 16KB PRG-ROM
        // $FFFA is at offset 16 + 0x3FFA = 16 + 16378 = 16394
        assert_eq!(output[16394], 0x34); // NMI low
        assert_eq!(output[16395], 0x12); // NMI high
        assert_eq!(output[16396], 0x78); // Reset low
        assert_eq!(output[16397], 0x56); // Reset high
        assert_eq!(output[16398], 0xBC); // IRQ low
        assert_eq!(output[16399], 0x9A); // IRQ high

        // Clean up
        fs::remove_file(test_file).ok();
        fs::remove_file("test_vectors.nes").ok();
    }
}
