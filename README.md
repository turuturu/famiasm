# famiasm

A 6502 assembler written in Rust for creating NES (Nintendo Entertainment System) ROM files.

## Overview

famiasm is a lightweight assembler that converts 6502 assembly source files (.asm) into NES ROM files (.nes) in iNES format. It's designed specifically for NES/Famicom homebrew development and is inspired by NESasm, aiming to provide a modern, Rust-based alternative while maintaining compatibility with existing assembly code.

**Note: This project is currently in beta. While functional, it may contain bugs and is still under active development.**

## Features

- Full 6502 instruction set support
- All standard addressing modes
- iNES format ROM generation
- Assembler directives (.inesprg, .ineschr, .bank, .org, .db, .dw, .incbin)
- Label and symbol resolution

## Installation

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Building from Source

```bash
git clone https://github.com/yourusername/famiasm.git
cd famiasm
cargo build --release
```

The compiled binary will be located at `target/release/famiasm`.

## Usage

```bash
famiasm <input.asm>
```

This will generate an output file with the same name as the input file but with a `.nes` extension.


## Assembly Language Syntax

### Basic Instructions

```asm
    LDA #$01        ; Load immediate value
    STA $2000       ; Store to memory
    JMP label       ; Jump to label
```

### Directives

```asm
    .inesprg 1      ; 1x 16KB PRG-ROM bank
    .ineschr 1      ; 1x 8KB CHR-ROM bank
    .inesmir 0      ; Horizontal mirroring
    .inesmap 0      ; Mapper 0 (NROM)

    .bank 0         ; Select bank 0
    .org $C000      ; Set origin address

    .db $01, $02    ; Define bytes
    .dw $1234       ; Define word (little-endian)
    .incbin "data.bin" ; Include binary file
```

### Labels

```asm
main:
    LDA #$00
    BEQ done        ; Branch to label
    JMP main
done:
    RTS
```

## Supported Instructions

famiasm supports the complete 6502 instruction set including:
- Transfer instructions (LDA, LDX, LDY, STA, STX, STY, TAX, TAY, TSX, TXA, TXS, TYA)
- Arithmetic operations (ADC, SBC, INC, INX, INY, DEC, DEX, DEY)
- Logic operations (AND, ORA, EOR)
- Shift and rotate (ASL, LSR, ROL, ROR)
- Compare and test (CMP, CPX, CPY, BIT)
- Branch instructions (BCC, BCS, BEQ, BMI, BNE, BPL, BVC, BVS)
- Jump and call (JMP, JSR, RTS, RTI)
- Stack operations (PHA, PHP, PLA, PLP)
- Flag operations (CLC, CLD, CLI, CLV, SEC, SED, SEI)
- Other (NOP, BRK)

## Addressing Modes

- Immediate: `LDA #$42`
- Zero Page: `LDA $42`
- Zero Page,X: `LDA $42,X`
- Zero Page,Y: `LDX $42,Y`
- Absolute: `LDA $1234`
- Absolute,X: `LDA $1234,X`
- Absolute,Y: `LDA $1234,Y`
- Indirect: `JMP ($1234)`
- Indexed Indirect: `LDA ($42,X)`
- Indirect Indexed: `LDA ($42),Y`
- Implied: `NOP`
- Accumulator: `ASL A`

## Development

### Building

```bash
cargo build         # Debug build
cargo build --release  # Release build
```

### Running Tests

```bash
cargo test
```

### Code Formatting

```bash
cargo fmt
```

## Project Structure

- `src/main.rs` - Entry point
- `src/assembler.rs` - Main assembler logic
- `src/tokenizer.rs` - Lexical analysis
- `src/parser.rs` - Syntax parsing
- `src/insts.rs` - 6502 instruction definitions
- `src/directive.rs` - Assembler directive handling
- `src/nes_header.rs` - iNES header generation
- `src/symbol_table.rs` - Label and symbol management

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
