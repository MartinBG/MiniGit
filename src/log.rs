//arguments: []

//load all commits file by file
//print them in order
extern crate walkdir;
use self::walkdir::WalkDir;
use std::path::PathBuf;
use ::commit_node::*;
use ::dir_ops::*;

pub fn log(start_path: &String) {
	let root = get_root_dir(start_path);
	let mut path_buf = PathBuf::from(&root);
	path_buf.push("_init_");
	path_buf.push("commits");

	let path = path_buf.as_path();

	for entry in WalkDir::new(&path) {
		let walk_dir = match entry {
			Ok(walk_dir) => walk_dir,
			Err(err) => panic!(err) 
		};

		let path = walk_dir.path();
		let path_str = path.to_str().unwrap();

		let split: Vec<&str> = path_str.split('\\').collect();
		let commit_name = String::from(*split.last().unwrap());

		if commit_name != "commits" {
			let commit = deserialize_commit(&root, &commit_name);
			commit.print();
		}
	}
}