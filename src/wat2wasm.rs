mod tokeniser;
mod opcodes;

pub fn write_wasm_preamble(buffer: &mut Vec<u8>) {
	// magic "\0asm"
	buffer.extend_from_slice(&[0x00, 0x61, 0x73, 0x6D]);
	// Version 1
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

enum Expression {
	Token(tokeniser::Token),
	Instruction(opcodes::Opcode),
	Expression(Box<Expression>)
}

fn parse_tokens(tokens: &[tokeniser::Token]) -> Result<Vec<Expression>, String> {
	let mut expressions = Vec::new();

	let mut i: usize = 0;
	while i < tokens.len() {
		match &tokens[i] {
			tokeniser::Token::Instruction(instruction) => {
				match opcodes::get_opcode(&instruction) {
					Some(opcode) => expressions.push(Expression::Instruction(opcode)),
					None => return Err(format!("Unknown instruction: {}", instruction))
				}
			},
			tokeniser::Token::Group(_) |
			tokeniser::Token::Identifier(_) => expressions.push(Expression::Token(tokens[i].clone())),
			_ => todo!("Have not yet implemented this token type")
		}

		i += 1;
	}

	Ok(expressions)
}

pub fn group_s_expressions(tokens: &mut Expression) {}

pub fn compile(source: &str) -> Vec<u8> {
	let mut buffer = Vec::new();

	let tokens = tokeniser::tokenise(source);

	println!("{:?}", tokens);

	buffer
}
