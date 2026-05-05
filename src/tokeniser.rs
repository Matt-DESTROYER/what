#[derive(Debug, PartialEq)]
pub enum TokenType {
	None,
	Instruction,
	Identifier,
	Literal,
	Separator,
	Group,
}

#[derive(Debug, PartialEq)]
pub enum Token {
	None,
	Instruction(String),
	Identifier(String),
	Literal(Literal),
	Separator(char),
	Group(char),
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum Literal {
	Integer(String),
	Float(String),
	String(String)
}

fn token_type(char: char) -> TokenType {
	if char == ',' {
		TokenType::Separator
	} else if char == '$' {
		TokenType::Identifier
	} else if char == '(' || char == ')' {
		TokenType::Group
	} else if char.is_ascii_digit() ||
			char == '.' ||
			char == '"' || char == '\'' {
		TokenType::Literal
	} else if char.is_ascii_alphabetic() {
		TokenType::Instruction
	} else {
		TokenType::None
	}
}

pub fn parse_token(raw_token: &str, token_type: TokenType) -> Token {
	if raw_token.is_empty() {
		return Token::None
	}

	match token_type {
		TokenType::Instruction => Token::Instruction(raw_token.to_owned()),
		TokenType::Identifier => Token::Identifier(raw_token.to_owned()),
		TokenType::Literal => if raw_token.chars().nth(0) == Some('"') &&
				raw_token.chars().nth_back(0) == Some('"') {
			let mut chars = raw_token.chars();
			chars.next();
			chars.next_back();
			Token::Literal(Literal::String(chars.as_str().to_owned()))
		} else if raw_token.contains('.') {
			Token::Literal(Literal::Float(raw_token.to_owned()))
		} else {
			Token::Literal(Literal::Integer(raw_token.to_owned()))
		},
		TokenType::Separator => Token::Separator(raw_token.chars().nth(0).unwrap_or('\0')),
		TokenType::Group => Token::Group(raw_token.chars().nth(0).unwrap_or('\0')),
		_ => Token::None
	}
}

fn push_token(tokens: &mut Vec<Token>, raw_token: &str, token_type: TokenType) {
	tokens.push(parse_token(raw_token, token_type));
}

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
						char == ',' {
					push_token(&mut tokens, &accumulator, accumulator_type);
					accumulator_type = token_type(char);
					accumulator = String::from(char);
				} else {
					accumulator.push(char);
				}
			},
			TokenType::Separator => accumulator_type = TokenType::None,
			TokenType::Group => {
				push_token(&mut tokens, &accumulator, accumulator_type);
				accumulator_type = token_type(char);
				accumulator = String::from(char);
			},
			TokenType::None => continue
		}
	}

	if accumulator_type != TokenType::None && !accumulator.is_empty() {
		push_token(&mut tokens, &accumulator, accumulator_type);
	}

	tokens
}
