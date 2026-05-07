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

struct Instruction {
	identifier: String,
	parameters: Vec<tokeniser::Literal>
}

#[allow(non_camel_case_types)]
enum WASMType {
	i32,
	i64,
	f32,
	f64
}

struct Variable {
	identifier: String,
	param_type: WASMType
}

struct TypeSignature {
	parameters: Vec<Variable>,
	return_types: Vec<WASMType>
}

struct Function {
	type_signature: TypeSignature,
	locals: Vec<Variable>,
	body: Vec<Instruction>
}

struct Global {
	identifier: String,
	global_type: WASMType,
	mutable: bool,
	initialisation_expression: Vec<Instruction>
}

struct Memory {
	minimum_size: u32,
	maximum_size: Option<u32>
}

struct Table {
	reference_type: WASMType,
	minimum_size: u32,
	maximum_size: Option<u32>
}

enum Entity {
	Function,
	Global,
	Memory,
	Table
}

enum ImportDescriptor {
	Function(TypeSignature),
	Global { global_type: WASMType, mutable: bool },
	Memory(Memory),
	Table(Table)
}

struct Import {
	module_name: String,
	field_name: String,
	descriptor: ImportDescriptor
}

struct Export {
	identifier: String,
	entity_type: Entity,
	index: u32
}

struct DataSegment {
	bytes: Vec<u8>,
	index: u32,
	initialisation: Vec<Instruction>
}

struct ElementSegment {
	functions: Vec<String>,
	index: u32,
	initialisation: Vec<Instruction>,
}

struct Module {
	data_segments: Vec<DataSegment>,
	element_segments: Vec<ElementSegment>,

	start_function: Option<String>,

	functions: Vec<Function>,
	globals: Vec<Global>,
	memory: Option<Memory>,
	tables: Vec<Table>,
	imports: Vec<Import>,
	exports: Vec<Export>
}
impl Module {
	pub fn new() -> Self {
		Self {
			data_segments: Vec::new(),
			element_segments: Vec::new(),
			start_function: None,
			functions: Vec::new(),
			globals: Vec::new(),
			memory: None,
			tables: Vec::new(),
			imports: Vec::new(),
			exports: Vec::new()
		}
	}
}

fn parse_tokens(tokens: &[tokeniser::Token]) -> Result<Module, String> {
	let mut module = Module::new();

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

pub fn group_s_expressions(tokens: &mut Expression) {

}

pub fn compile(source: &str) -> Vec<u8> {
	let mut buffer = Vec::new();

	let tokens = tokeniser::tokenise(source);

	println!("{:?}", tokens);

	buffer
}
