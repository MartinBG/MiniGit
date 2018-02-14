// use std::env;
extern crate walkdir;
use walkdir::WalkDir;

// https://doc.rust-lang.org/book/second-edition/ch12-03-improving-error-handling-and-modularity.html
// this is for command creating from command line arguments and error handling

extern crate MiniGit;
use MiniGit::commit_node::*;
#[macro_use] 
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate time;
use time::now;


fn main() {
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}",MiniGit::command_parsing::parse_command(&args).unwrap());
    let dirTree = MiniGit::dir_ops::DirTree::new(&String::from("C:\\MyThings\\rust\\MiniGit\\testDir"));
    let commit = CommitNode::new(&String::from("C:\\MyThings\\rust\\MiniGit\\testDir"), &String::from("Initial commit"));
    // let commits = vec!(commit);
    // let commitVec = Commits {
    // 	commit_vec: commits
    // };
    // print!("{:?}", commitVec);
    let h = SerializeCommit(&String::from("C:\\MyThings\\rust\\MiniGit\\testDir"), &commit);
    println!("{:?}", commit);
    let des = DeserializeCommit(&String::from("C:\\MyThings\\rust\\MiniGit\\testDir"), &String::from(h));
    println!("{:?}", des);
    // let h = SerializeCommit(&String::from("C:\\MyThings\\rust\\MiniGit\\testDir", &commit)
    // let a = MiniGit::dir_ops::get_root_dir(&String::from("C:\\MyThings\\rust\\MiniGit"));
    // print!("{:?}", a);
 //    let str_vec = vec!(String::from("a"), String::from("b"), String::from("c"));
	// let serialized = serde_json::to_string(&str_vec).unwrap();
	// let des: Vec<String> = serde_json::from_str(&serialized).unwrap();

	// println!("{:?}", TmToString(&now()));

}