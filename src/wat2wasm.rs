mod tokeniser;
mod opcodes;

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

#[derive(Debug)]
enum AbstractSyntax {
	Tokens(tokeniser::Token),
	Group(Vec<AbstractSyntax>)
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

	fn recursive_ast(tokens: &[tokeniser::Token]) -> Result<AbstractSyntax, String> {
		let mut ast = AbstractSyntax::Group(Vec::new());

		let mut i: usize = 0;
		while i < tokens.len() {
			if let tokeniser::Token::Group(group) = &tokens[i] {
				if *group == '(' {
					let group_start = i;

					let mut depth = 1;
					loop {
						i += 1;
						if i >= tokens.len() {
							break;
						}

						if let tokeniser::Token::Group(group) = &tokens[i] {
							if *group == '(' {
								depth += 1;
							} else if *group == ')' {
								depth -= 1;

								if depth == 0 {
									break;
								}
							}
						}
					}

					if let tokeniser::Token::Group(group) = &tokens[i] && *group == ')' {
						let slice = &tokens[group_start + 1..i];
						if let AbstractSyntax::Group(ast) = &mut ast {
							match Module::recursive_ast(slice) {
								Ok(group) => ast.push(group),
								Err(err) => return Err(err)
							}
						}
					}
				} else if *group == ')' {
					return Err("Unmatched bracket".to_owned())
				}
			} else if let AbstractSyntax::Group(ast) = &mut ast {
				ast.push(AbstractSyntax::Tokens(tokens[i].clone()));
			}

			i += 1;
		}

		Ok(ast)
	}

	fn parse(&mut self, source: &str) -> Result<(), String> {
		let mut tokens = tokeniser::tokenise(source);

		let ast = Module::recursive_ast(&mut tokens);
		println!("{:#?}", ast);

		Ok(())
	}

	pub fn compile(&mut self, source: &str) -> Result<Vec<u8>, String> {
		let mut bytes = Vec::new();

		match self.parse(source) {
			Ok(_) => {},
			Err(err) => return Err(err)
		}

		Ok(bytes)
	}
}

fn write_wasm_preamble(buffer: &mut Vec<u8>) {
	// magic "\0asm"
	buffer.extend_from_slice(&[0x00, 0x61, 0x73, 0x6D]);

	// Version 1
	buffer.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]);
}

pub fn compile(source: &str) -> Result<Vec<u8>, String> {
	let mut preamble = Vec::new();
	write_wasm_preamble(&mut preamble);

	match Module::new().compile(source) {
		Ok(bytes) => {
			preamble.extend(bytes);
			Ok(preamble)
		},
		Err(err) => Err(err)
	}
}
