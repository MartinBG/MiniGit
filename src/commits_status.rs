use ::dir_ops::*;
use std::path::{Path, PathBuf};
use ::{serde_json,serde_derive};

#[derive(Serialize, Deserialize, Debug)]
pub struct CommitsStatus {
	pub current_commit: String,
	pub head_commit: String
}

impl CommitsStatus {
	pub fn new() -> Self {
		CommitsStatus {
			current_commit: String::new(),
			head_commit: String::new()
		}
	}

	pub fn load(start_dir: &String) -> Self {
		let root = get_root_dir(start_dir);
		let mut path = PathBuf::from(&root);
		path.push("_init_");
		path.push("commits_status.json");

		let content = get_file_content(&path);
		let commitsStatus: Self = match serde_json::from_str(&content) {
			Ok(res) => res,
			Err(err) => panic!(err)
		};

		commitsStatus
	}

	pub fn save(&self, start_dir: &String) {
		let root = get_root_dir(start_dir);
		let mut path = PathBuf::from(&root);
		path.push("_init_");	
		path.push("commits_status.json");

		let commits_status_str: String = serde_json::to_string(self).unwrap();
		write_to_file(&path, &commits_status_str);
	}
}
