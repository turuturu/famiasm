#[cfg(test)]
mod incbin_tests {
    use std::fs;
    use std::process::Command;

    fn run_famiasm(input_file: &str) -> Result<Vec<u8>, String> {
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
    fn test_incbin_relative_path() {
        // Create test directory structure
        fs::create_dir_all("test_incbin_dir").unwrap();
        
        // Create a binary file to include
        let test_data = vec![0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE];
        fs::write("test_incbin_dir/test.bin", &test_data).unwrap();
        
        // Create ASM file that uses INCBIN with relative path
        let test_asm = r#"
    .inesprg 1
    .ineschr 0
    .inesmir 1
    .inesmap 0

    .bank 0
    .org $C000
    
    .incbin "test.bin"

START:
    JMP START

    .org $FFFA
    .dw 0
    .dw START
    .dw 0
"#;
        fs::write("test_incbin_dir/test_incbin.asm", test_asm).unwrap();
        
        // Run the assembler
        let result = run_famiasm("test_incbin_dir/test_incbin.asm");
        assert!(result.is_ok(), "Failed to assemble with INCBIN");
        
        let output = result.unwrap();
        
        // Verify the included data is in the output
        // PRG-ROM starts at offset 16 (after header)
        for (i, &expected) in test_data.iter().enumerate() {
            assert_eq!(output[16 + i], expected, 
                "Mismatch at offset {}: expected 0x{:02X}, got 0x{:02X}", 
                i, expected, output[16 + i]);
        }
        
        // Clean up
        fs::remove_dir_all("test_incbin_dir").ok();
        fs::remove_file("test_incbin_dir.nes").ok();
    }

    #[test]
    fn test_incbin_subdirectory() {
        // Create nested directory structure
        fs::create_dir_all("test_incbin_nested/data").unwrap();
        
        // Create a binary file in subdirectory
        let test_data = vec![0x12, 0x34, 0x56, 0x78];
        fs::write("test_incbin_nested/data/graphics.chr", &test_data).unwrap();
        
        // Create ASM file that uses INCBIN with subdirectory path
        let test_asm = r#"
    .inesprg 1
    .ineschr 0
    .inesmir 1
    .inesmap 0

    .bank 0
    .org $C000
    
    .incbin "data/graphics.chr"

START:
    JMP START

    .org $FFFA
    .dw 0
    .dw START
    .dw 0
"#;
        fs::write("test_incbin_nested/test_nested.asm", test_asm).unwrap();
        
        // Run the assembler
        let result = run_famiasm("test_incbin_nested/test_nested.asm");
        assert!(result.is_ok(), "Failed to assemble with nested INCBIN");
        
        let output = result.unwrap();
        
        // Verify the included data
        for (i, &expected) in test_data.iter().enumerate() {
            assert_eq!(output[16 + i], expected);
        }
        
        // Clean up
        fs::remove_dir_all("test_incbin_nested").ok();
        fs::remove_file("test_incbin_nested.nes").ok();
    }
}