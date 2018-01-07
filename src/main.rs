// use std::env;
extern crate walkdir;
use walkdir::WalkDir;

// https://doc.rust-lang.org/book/second-edition/ch12-03-improving-error-handling-and-modularity.html
// this is for command creating from command line arguments and error handling

extern crate MiniGit;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}",MiniGit::command_parsing::parse_command(&args).unwrap());
    let a = MiniGit::DirTree::DirTree::new(&String::from("C:\\MyThings\\rust\\MiniGit\\testDir"));
    print!("{:?}", a);

}