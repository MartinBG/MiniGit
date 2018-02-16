//arguments: commitId(will be hash) filename
// get specific file from another commit

//1) load commits desired and current
//2) find file in dirTree
//3) check exist and if does check hash and if equal do nothing other case del and add

use ::dir_ops::*;
use ::commits_status::*;
use ::commit_node::*;
use std::path::PathBuf;

pub fn cherryPick(start_path: &String, commit_id: &String, file_path: &String) {
	let root = get_root_dir(start_path);

	let desired_commit = DeserializeCommit(&root, commit_id);

	let found_path: Vec<&DirItem> = desired_commit.dir_state.dir_items
									.iter()
								    .skip_while(|x| x.path != *file_path)
								    .collect();
	if found_path.len() == 0 {
		print!("No such file path found in desired commit!");
	} else {
		let current_dir_state = DirTree::new(&root);

		let found_path_current: Vec<&DirItem> = current_dir_state.dir_items
										.iter()
									    .skip_while(|x| x.path != *file_path)
									    .collect();
		if found_path_current.len() == 0 || 
		   found_path_current.iter().nth(0).unwrap().hash != found_path.iter().nth(0).unwrap().hash {
		   	let content = get_from_db(&root, &found_path.iter().nth(0).unwrap().hash);
			let path_buff = PathBuf::from(file_path); 
		   	write_to_file(&path_buff, &content);
		}
	}
}