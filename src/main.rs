// use std::env;
extern crate walkdir;
use walkdir::WalkDir;

// https://doc.rust-lang.org/book/second-edition/ch12-03-improving-error-handling-and-modularity.html
// this is for command creating from command line arguments and error handling

extern crate MiniGit;
use MiniGit::commit_node::*;
#[macro_use] 
extern crate serde;



fn main() {
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}",MiniGit::command_parsing::parse_command(&args).unwrap());
    let dirTree = MiniGit::dir_ops::DirTree::new(&String::from("C:\\MyThings\\rust\\MiniGit\\testDir"));
    let commit = CommitNode::new(1, &String::from("C:\\MyThings\\rust\\MiniGit\\testDir"), &String::from("Initial commit"));
    let commits = vec!(commit);
    let commitVec = Commits {
    	commit_vec: commits
    };
    // print!("{:?}", commitVec);
    SerializeCommits(&String::from("C:\\MyThings\\rust\\MiniGit\\testDir"), &commitVec);
    //let des = DeserializeCommits(&String::from("C:\\MyThings\\rust\\MiniGit\\testDir"));

    //commit_node::SerializeCommits()
    // let a = MiniGit::dir_ops::get_root_dir(&String::from("C:\\MyThings\\rust\\MiniGit"));
    // print!("{:?}", a);

}