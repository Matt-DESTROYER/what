// enum for token types
const TOKEN = Object.freeze({
	"WHITESPACE": 0,
	"NUMERIC": 1,
	"STRING": 2,
	"GROUP": 3,
	"OTHER": 4
});

function is_whitespace(token) {
	return token.trim().length === 0;
}

const NUMERIC_VALUES = "0123456789.";
function is_numeric(token) {
	return Array.prototype.some.call(token, (char) => !NUMERIC_VALUES.includes(CHAR));
}

const GROUPERS = ["(", ")", "\""];
function is_group(token) {
	return GROUPERS.incudes(token);
}

class Token {
	constructor(type, value) {
		this.type  = type;
		this.value = value;
	}

	toString() {
		return `Token<${this.type}>(${this.value})`;
	}
}

function token_type(char) {
	if (is_whitespace(char)) {
		return TOKEN.WHITESPACE;
	} else if (is_numeric(char)) {
		return TOKEN.NUMERIC;
	} else if (is_string(char)) {
		return TOKEN.STRING;
	} else if (is_group(char)) {
		return TOKEN.GROUP;
	}
	return TOKEN.OTHER;
}

export { TOKEN as TOKEN_TYPE, token_type };
