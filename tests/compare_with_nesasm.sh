#!/bin/bash

# Usage: ./compare_with_nesasm.sh input.asm

if [ $# -ne 1 ]; then
    echo "Usage: $0 input.asm"
    exit 1
fi

INPUT_FILE="$1"
BASE_NAME=$(basename "$INPUT_FILE" .asm)
FAMIASM_OUTPUT="${BASE_NAME}_famiasm.nes"
NESASM_OUTPUT="${BASE_NAME}_nesasm.nes"

echo "Compiling with famiasm..."
cargo run -- "$INPUT_FILE"
if [ $? -ne 0 ]; then
    echo "famiasm compilation failed"
    exit 1
fi
mv "${BASE_NAME}.nes" "$FAMIASM_OUTPUT"

echo "Compiling with nesasm..."
nesasm "$INPUT_FILE" -o "$NESASM_OUTPUT"
if [ $? -ne 0 ]; then
    echo "nesasm compilation failed"
    echo "Make sure nesasm is installed and in your PATH"
    exit 1
fi

echo "Comparing outputs..."
if cmp -s "$FAMIASM_OUTPUT" "$NESASM_OUTPUT"; then
    echo "Success: Files are identical"
else
    echo "Error: Files differ"
    echo "Running hexdump diff:"
    diff <(hexdump -C "$FAMIASM_OUTPUT") <(hexdump -C "$NESASM_OUTPUT") | head -20
    echo ""
    echo "File sizes:"
    ls -la "$FAMIASM_OUTPUT" "$NESASM_OUTPUT"
fi