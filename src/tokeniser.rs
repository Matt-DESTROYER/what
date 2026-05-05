use std::os::raw;

#[derive(PartialEq)]
pub enum TokenType {
	None,
	Instruction,
	Identifier,
	Literal,
	Separator,
	Group,
}

pub enum Token {
	None,
	Instruction(String),
	Identifier(String),
	Literal(Literal),
	Separator(char),
	Group(char),
}

#[allow(non_camel_case_types)]
pub enum Literal {
	i32(String),
	i64(String),
	u32(String),
	u64(String)
}

fn token_type(char: char) -> TokenType {
	if char.is_whitespace() {
		TokenType::Separator
	} else if char == '$' {
		TokenType::Identifier
	} else if char == '{' || char == '"' || char == '\'' {
		TokenType::Group
	} else if char.is_ascii_digit() || char == '.' {
		TokenType::Literal
	} else if char.is_ascii_alphabetic() {
		TokenType::Instruction
	} else {
		TokenType::None
	}
}

fn parse_token(raw_token: &str, token_type: TokenType) -> Token {
	if raw_token.is_empty() {
		return Token::None
	}

	match token_type {
		TokenType::Instruction => Token::Instruction(raw_token.to_owned()),
		TokenType::Identifier => Token::Identifier(raw_token.to_owned()),
		TokenType::Literal => todo!("Need to look backwards to determine literal type..."),
		TokenType::Separator => Token::Separator(raw_token.chars().nth(0).unwrap_or('\0')),
		TokenType::Group => Token::Group(raw_token.chars().nth(0).unwrap_or('\0')),
		_ => Token::None
	}
}

fn push_token(tokens: &mut Vec<Token>, raw_token: &str, token_type: TokenType) {}

pub fn tokenise(buffer: &str) -> Vec<Token> {
	let mut tokens = Vec::new();

	let mut accumulator = String::new();
	let mut accumulator_type = TokenType::None;

	for char in buffer.chars() {
		if accumulator_type == TokenType::None {
			accumulator_type = token_type(char);
			accumulator = String::from(char);
			continue;
		}

		match accumulator_type {
			TokenType::Instruction |
			TokenType::Identifier |
			TokenType::Literal => {
				if char.is_whitespace() ||
						char == '(' || char == ')' ||
						char == '"' || char == '\'' ||
						char == ',' {
					push_token(&mut tokens, &accumulator, accumulator_type);
					accumulator_type = TokenType::None;
				} else {
					accumulator.push(char);
				}
			},
			TokenType::Separator |
			TokenType::Group => {
				push_token(&mut tokens, &accumulator, accumulator_type);
				accumulator_type = TokenType::None;
			},
			TokenType::None => continue
		}
	}

	if accumulator_type != TokenType::None && !accumulator.is_empty() {
		push_token(&mut tokens, &accumulator, accumulator_type);
	}

	tokens
}
