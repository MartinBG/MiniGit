
use std::path::PathBuf;
use ::serde_json;
use std::error::Error;
extern crate time;
use self::time::{Tm,now};
use ::dir_ops::*;
use ::dir_structs::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct CommitNode {
	pub commit_time: String,
	pub message: String,
	pub dir_state: DirTree
}

impl CommitNode {
	pub fn new(root_dir: &String, message: &String) -> Self {
		CommitNode {
			commit_time: tm_to_string(&now()),
			message: message.clone(),
			dir_state: DirTree::new(root_dir)
		}
	}

	pub fn print(&self) {
		println!("\ncommit id: commit_{}.json", self.commit_time);
		println!("commit message: {:?}", self.message);
		println!("commited time: {}", self.commit_time);

	}
}

pub fn tm_to_string(time: &Tm) -> String {
	let mut str_time: Vec<String> = Vec::new();
	str_time.push((time.tm_year + 1900).to_string());
	str_time.push((time.tm_mon + 1).to_string());
	str_time.push(time.tm_mday.to_string());
	str_time.push(time.tm_hour.to_string());
	str_time.push(time.tm_min.to_string());
	str_time.push(time.tm_sec.to_string());
	str_time.join(".")
}

pub fn deserialize_commit(start_dir: &String, commit_id: &String) -> CommitNode {
	let root_dir = get_root_dir(start_dir);
	let mut dir_path_buf = PathBuf::from(&root_dir);
	dir_path_buf.push("_init_");
	dir_path_buf.push("commits");
	dir_path_buf.push(&commit_id);
	let commit_str = get_file_content(&dir_path_buf);
	let commit: CommitNode = match serde_json::from_str(&commit_str) {
		Ok(res) => res,
		Err(err) => panic!(String::from(err.description()))
	};
	commit
} 

pub fn serialize_commit(start_dir: &String, commit: &CommitNode) -> String {
	let root_dir = get_root_dir(start_dir);
	let mut dir_path_buf = PathBuf::from(&root_dir);
	dir_path_buf.push("_init_");
	dir_path_buf.push("commits");

	let serialized_commit = serde_json::to_string(&commit).unwrap();
	let mut commit_name = String::from("commit_");
	commit_name.push_str(&commit.commit_time);
	commit_name.push_str(".json");
	dir_path_buf.push(&commit_name);
	
	write_to_file(&dir_path_buf, &serialized_commit);

	commit_name
}
