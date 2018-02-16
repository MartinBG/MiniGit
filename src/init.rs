//arguments: []
//generate starting files

//create folder _init_
//create folder db
//create file status_info
// first commit should be name head and it is going to be overwrited

use std::path::PathBuf;
use ::commits_status::*;
use std::fs::create_dir;

pub fn init(start_path: &String) {
	let mut path_buf = PathBuf::from(start_path);
	path_buf.push("_init_");
	let cloned_path = path_buf.clone();
	let path = cloned_path.as_path();
	if path.is_dir() {
		println!("There is already defined a repository!");
		return;
	} else {
		create_dir(&path_buf).expect("could not create directory");;
		
		path_buf.push("db");
		create_dir(&path_buf).expect("could not create directory");
		path_buf.pop();
		
		path_buf.push("commits");
		create_dir(&path_buf).expect("could not create directory");
		path_buf.pop();

		let commits_status = CommitsStatus::new();
		commits_status.save(start_path);
	}
}