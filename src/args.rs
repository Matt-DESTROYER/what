use std::env;

#[derive(Clone, Debug)]
pub struct Arguments {
	unnamed_arguments: Vec<String>,
	named_arguments: Vec<String>
}
impl Arguments {
	pub fn new() -> Self {
		Self {
			unnamed_arguments: Vec::new(),
			named_arguments: Vec::new()
		}
	}
	pub fn get_unnamed(&self) -> &Vec<String> {
		&self.unnamed_arguments
	}
	pub fn check_flag(&self, flag: &str) -> bool {
		self.named_arguments.contains(&flag.to_owned())
	}
}

pub fn get_cli_arguments() -> Arguments {
	let mut cli_args = Arguments::new();

	let args: Vec<String> = env::args().collect();
	for arg in args.iter().skip(1) {
		if arg.len() > 2 && arg.starts_with('-') && !arg.starts_with("--") {
			for ch in arg.chars().skip(1) {
				cli_args.named_arguments.push(format!("-{}", ch));
			}
		} else if !arg.starts_with('-') {
			cli_args.unnamed_arguments.push(arg.to_owned());
		} else {
			cli_args.named_arguments.push(arg.to_owned());
		}
	}

	cli_args
}
