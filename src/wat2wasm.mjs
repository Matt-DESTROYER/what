import { argv } from "node:process";

import OPCODES from "./opcodes.mjs";
import tokenise from "./tokenise.mjs"
import { TOKEN_TYPE, token_type } from "./wat.mjs";

function compile(source) {
    const tokens = tokenise(source, token_type);

}

const arguments = argv.slice(2);
