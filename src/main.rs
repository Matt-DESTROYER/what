use std::fs;

mod args;

mod leb128;
mod wat2wasm;

fn print_help() {
	println!("Usage:");
	println!("what <flags> <file.ext>");
	println!();
	println!("Flags:");
	println!("--wat2wasm - compiles the input WAT file to WASM");
	println!();
}

fn main() {
	let cli_args = args::get_cli_arguments();

	if cli_args.unnamed_arg_count() == 0 && cli_args.flag_arg_count() == 0 {
		println!("No arguments supplied");

		print_help();
		return;
	}

	if cli_args.check_flag("--help") || cli_args.check_flag("-h") {
		print_help();
	}

	if cli_args.check_flag("--wat2wasm") {
		if let Some(file) = cli_args.get_unnamed().last() {
			let code = match fs::read_to_string(file) {
				Ok(code) => code,
				Err(err) => return eprintln!("Error: {}", err),
			};

			println!("{}", code);
		}
	}
}
