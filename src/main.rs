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
use MiniGit::dir_ops::*;
use std::fs::File;
use MiniGit::init::*;
use MiniGit::commit::*;
use MiniGit::status::*;
use MiniGit::log::*;

fn main() {

   	// init(&String::from("C:\\MyThings\\rust\\MiniGit\\testDir"));
   	//commit(&String::from("C:\\MyThings\\rust\\MiniGit\\testDir"), &String::from("Second commit"));
   	//status(&String::from("C:\\MyThings\\rust\\MiniGit\\testDir"));
   	log(&String::from("C:\\MyThings\\rust\\MiniGit\\testDir"));





    // let args: Vec<String> = env::args().collect();
    // println!("{:?}",MiniGit::command_parsing::parse_command(&args).unwrap());
    // let dirTree = MiniGit::dir_ops::DirTree::new(&String::from("C:\\MyThings\\rust\\MiniGit\\testDir"));
    //dirTree.print();
    // let dirTree2 = MiniGit::dir_ops::DirTree::new(&String::from("C:\\MyThings\\rust\\MiniGit\\testDir\\dir_lvl1"));
    // dirTree2.print();
    // let dif = dirTree.differences(&dirTree2);
   	// dif.print();


   	//let mut f = File::create(&&String::from("C:\\MyThings\\rust\\MiniGit\\testDir\\new\\old\\magic\\secrets.txt")).expect("could not create file");
   	
   	// println!("{:?}", dirTree.differences(&dirTree2));
    // let commit = CommitNode::new(&String::from("C:\\MyThings\\rust\\MiniGit\\testDir"), &String::from("Initial commit"));
    // let commits = vec!(commit);
    // let commitVec = Commits {
    // 	commit_vec: commits
    // };
    // print!("{:?}", commitVec);
    // let h = SerializeCommit(&String::from("C:\\MyThings\\rust\\MiniGit\\testDir"), &commit);
    // println!("{:?}", commit);
    // let des = DeserializeCommit(&String::from("C:\\MyThings\\rust\\MiniGit\\testDir"), &String::from(h));
    // println!("{:?}", des);
    // let h = SerializeCommit(&String::from("C:\\MyThings\\rust\\MiniGit\\testDir", &commit)
    // let a = MiniGit::dir_ops::get_root_dir(&String::from("C:\\MyThings\\rust\\MiniGit"));
    // print!("{:?}", a);
 //    let str_vec = vec!(String::from("a"), String::from("b"), String::from("c"));
	// let serialized = serde_json::to_string(&str_vec).unwrap();
	// let des: Vec<String> = serde_json::from_str(&serialized).unwrap();

	// println!("{:?}", TmToString(&now()));

	// let z = [1,2];
	// let mut i = z.iter();
	// let a = i.next();
	// let b = i.next();
	// print!("{:?}", i.next());
	// println!("{:?}", "C:\\MyThings\\rust\\MiniGit\\testDir\\dir_lvl0" > "C:\\MyThings\\rust\\MiniGit\\testDir\\dir_lvl1");
}