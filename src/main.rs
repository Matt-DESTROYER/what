mod args;
mod tokeniser;
mod opcodes;

fn main() {
    let cli_args = args::get_cli_arguments();

    println!("{:#?}", cli_args);
}
