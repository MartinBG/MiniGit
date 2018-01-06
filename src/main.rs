use std::env;

// https://doc.rust-lang.org/book/second-edition/ch12-03-improving-error-handling-and-modularity.html
// this is for command creating from command line arguments and error handling

extern crate MiniGit;

fn main() {
	//let a: [String] = &[String::from("asd"), String::from("dasd"), String::from("dsasd")];
	//print!("{:?}", a.len());
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    println!("{:?}",MiniGit::command_parsing::parse_command(&args).unwrap());
}