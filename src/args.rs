use std::env;

#[derive(Clone, Debug)]
pub struct Arguments {
	unnamed_arguments: Vec<String>
}
impl Arguments {
	pub fn new() -> Self {
		Self {
			unnamed_arguments: Vec::new()
		}
	}
	pub fn get_unnamed(&self) -> &Vec<String> {
		&self.unnamed_arguments
	}
}

pub fn get_cli_arguments() -> Arguments {
	let mut cli_args = Arguments::new();

	let args: Vec<String> = env::args().collect();
	let mut expanded_args: Vec<String> = Vec::new();
	for arg in args.iter().skip(1) {
		if arg.len() > 2 && arg.starts_with('-') && !arg.starts_with("--") {
			for ch in arg.chars().skip(1) {
				expanded_args.push(format!("-{}", ch));
			}
		} else if !arg.starts_with('-') {
			cli_args.unnamed_arguments.push(arg.to_owned());
		} else {
			expanded_args.push(arg.to_owned());
		}
	}

	for arg in expanded_args {
		match arg.as_str() {
			_ => {
			}
		}
	}

	cli_args
}
