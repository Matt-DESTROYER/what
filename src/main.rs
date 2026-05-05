use std::fs;

mod args;
mod tokeniser;
mod opcodes;

fn main() {
	let cli_args = args::get_cli_arguments();

	if cli_args.check_flag("--wat2wasm") {
		if let Some(file) = cli_args.get_unnamed().last() {
			let code = match fs::read_to_string(file) {
				Ok(code) => code,
				Err(err) => return eprintln!("Error: {}", err)
			};

			let tokens = tokeniser::tokenise(&code);
			for token in tokens {
				println!("{:?}", token);
			}
		}
	}
}
