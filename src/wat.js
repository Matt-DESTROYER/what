
// enum for token types
const TOKEN = Object.freeze({
	"WHITESPACE": 0,
	"NUMBER": 1,
	"STRING": 2,
	"GROUP": 3,
	"OTHER": 4
});

function is_whitespace(token) {
	return token.trim().length === 0;
}

function is_number(token) {
	return !isNaN(Number(token));
}

function is_string(token) {
	return token.startsWith("\"");
}

function is_group(token) {
	return token.startsWith("(");
}

class Token {
	constructor(type, value) {
		this.type = type;
		this.value = value;
	}

	toString() {
		return `Token(${this.type}, ${this.value})`;
	}
}

const tokenise = function tokenise(stream) {
	const tokens = [];

	let current_token = "";
	let current_token_type = null;
	for (const char of stream) {
	}
	if (current_token) {
		tokens.push(new Token(TOKEN.OTHER, current_token));
	}
	return tokens;
};

export { tokenise };
