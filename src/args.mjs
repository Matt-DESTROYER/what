import { argv } from "node:process";

function is_arg(str) {
	return str.startsWith("-") || str.startsWith("--");
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

	console.log(args);

	for (let i = 0; i < args.length; i++) {
		if (is_arg(args[i])) {
			const arg = strip_arg_name(args[i]);
			
		}
	}

    return arg_results;
}

export { get_args as default };
