import OPCODES from "./opcodes.js";
import tokenise from "./tokenise.js"
import { TOKEN_TYPE, token_type } from "./wat.js";

function compile(source) {
    const tokens = tokenise(source, token_type);
}
