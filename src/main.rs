extern crate MiniGit;
use std::env;
use MiniGit::command_parsing::*;
use std::process;


fn main() {
	
	let curr_dir = env::current_dir().unwrap_or_else(|err| {
		eprintln!("{}", err);
		process::exit(1);
	});
	
	let args: Vec<String> = env::args().collect();
	let command = parse_command(&args).unwrap_or_else(|err| {
		eprintln!("{}", err);
		process::exit(1);
	});

	let path = curr_dir.as_path();
	let path_str = path.to_str().unwrap();

	run_command(command, &String::from(path_str));
}