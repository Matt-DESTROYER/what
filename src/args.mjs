import { argv } from "node:process";

function is_arg(str) {
	return str.startsWith("-") || str.startsWith("--");
}

function is_flag(arg, next) {
	return is_arg(arg) && !is_arg(next);
}

function strip_arg_name(arg_name) {
	let start = 0;
	while (arg_name[start] === "-") {
		start++;
	}
	return arg_name.substring(start);
}

function get_args() {
    const args = argv.slice(2);

	const arg_results = Object.seal({
		named: {},
		unnamed: []
	});

	for (let i = 0; i < args.length; i++) {
		if (args < args.length - 1 &&
			is_flag(args[i], args[i + 1])) {
			const arg = args[i].split("=");
			arg[0] = strip_arg_name(arg[0]);
			if (arg.length === 1) {
				arg_results.named[args[i]] = true;
			} else {
				arg_results.named[arg[0]] = arg[1];
			}
		} else if (is_arg(args[i])) {
			const arg = args[i].split("=");
			arg[0] = strip_arg_name(arg[0]);
			if (arg.length === 1) {
				arg_results.named[args[i]] = true;
			} else {
				arg_results.named[arg[0]] = arg[1];
			}
		}
	}

    return arg_results;
}

export { getArgs as default };
