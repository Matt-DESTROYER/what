mod tokeniser;
mod opcodes;

pub fn write_wasm_preamble(buffer: &mut Vec<u8>) {
	// \0asm
	buffer.extend_from_slice(&[0x00, 0x61, 0x73, 0x6D]);
	// Version: 1
	buffer.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]);
}

/*
 * Section IDs:
 *  1. Type section
 *  3. Function section
 *  7. Export section
 * 10. Code section
 */

/*
 * Section format:
 * Section ID: 1 byte
 * Section size: size of section payload in bytes encoded as a ULEB128 u32
 * Payload: the contents of the section
 */
