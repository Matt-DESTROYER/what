import OPCODES from "./opcodes.mjs";
import tokenise from "./tokenise.mjs"
import { TOKEN_TYPE, token_type } from "./wat.mjs";
import getArgs from "./args.mjs";

function compile(source) {
    const args = getArgs();

    const tokens = tokenise(source, token_type);
}



