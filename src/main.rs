use clap::{Arg, Command};
use std::fs;

fn app() -> Command {
	Command::new(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.arg_required_else_help(true)
		.arg(Arg::new("input").help("JSON file to convert to TOML").index(1))
}
fn main() {
	let matches = app().get_matches();
	let input_file = matches.get_one::<String>("input").unwrap();
	let input_data = if let Ok(data) = fs::read(input_file) {
		data
	} else {
		eprintln!("Error: failure reading '{input_file}'");
		return;
	};
	let toml_data: toml::Value = serde_json::from_slice(&input_data).unwrap();
	print!("{toml_data}");
}
