use ::dir_ops::*;

use std::path::PathBuf;
use ::{serde_json,serde_derive};
use std::error::Error;


#[derive(Serialize, Deserialize, Debug)]
pub struct CommitNode {
	id: u32,
	//commit_time: String,
	message: String,
	dir_state: DirTree
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Commits {
	pub commit_vec: Vec<CommitNode>
}

impl CommitNode {
	pub fn new(id: u32, root_dir: &String, message: &String) -> Self {
		CommitNode {
			id: id,
			//commit_time: Utc::now(),
			message: message.clone(),
			dir_state: DirTree::new(root_dir)
		}
	}
}

pub fn DeserializeCommits(start_dir: &String) -> Vec<CommitNode> {
	let root_dir = get_root_dir(start_dir);
	let mut dir_path_buf = PathBuf::from(&root_dir);
	dir_path_buf.push("_init_");
	dir_path_buf.push("commits.json");
	// let commits_path = dir_path_buf.as_path();
	println!("before read");
	let commits_str = get_file_content(&dir_path_buf);
	print!("before des");
	let commits: Vec<CommitNode> = match serde_json::from_str(&commits_str) {
		Ok(res) => res,
		Err(err) => panic!(String::from(err.description()))
	};
	println!("after des");
	commits
} 

pub fn SerializeCommits(start_dir: &String, commits: &Commits) {
	let root_dir = get_root_dir(start_dir);
	let mut dir_path_buf = PathBuf::from(&root_dir);
	dir_path_buf.push("_init_");
	dir_path_buf.push("commits.json");
	//let commits_path = dir_path_buf.as_path();
	let serialized = serde_json::to_string(&commits).unwrap();
	println!("{:?}", serialized);
	// write_to_file(&dir_path_buf, &serialized);
	let commits: Vec<CommitNode> = serde_json::from_str(&serialized).unwrap();
	println!("{:?}", &commits);
}




// 
// struct Point {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     let point = Point { x: 1, y: 2 };

//     // Convert the Point to a JSON string.
//     let serialized = serde_json::to_string(&point).unwrap();

//     // Prints serialized = {"x":1,"y":2}
//     println!("serialized = {}", serialized);

//     // Convert the JSON string back to a Point.
//     let deserialized: Point = serde_json::from_str(&serialized).unwrap();

//     // Prints deserialized = Point { x: 1, y: 2 }
//     println!("deserialized = {:?}", deserialized);
// }