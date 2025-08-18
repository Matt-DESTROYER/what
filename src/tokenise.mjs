const tokenise = function tokenise(stream, token_type) {
	const tokens = [];

	let current_token = "";
	let current_token_type = null;
	for (const char of stream) {
		const new_token_type = token_type(char);
		if (current_token_type !== new_token_type) {
			if (current_token !== "") {
				tokens.push(new Token(current_token_type, current_token));
			}
			current_token_type = new_token_type;
			current_token = char;
		} else {
			current_token += char;
		}
	}
	if (current_token) {
		tokens.push(new Token(TOKEN.OTHER, current_token));
	}
	return tokens;
};

export { tokenise as default };
