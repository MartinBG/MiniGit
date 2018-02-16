//obvious - message
use ::dir_ops::*;
use ::commit_node::*;
use ::commits_status::*;
use std::path::PathBuf;

pub fn commit(start_path: &String, message: &String) {
	let root = get_root_dir(start_path);

	let mut commits_status = CommitsStatus::load(&root);
	if commits_status.current_commit == commits_status.head_commit {

		let commit = CommitNode::new(&root, message);

		for f in commit.dir_state.dir_items.iter() {
			if f.hash != "" && !exists_in_db(&root, &f.hash)  {
				let path_buf = PathBuf::from(&f.path);
				let contents = get_file_content(&path_buf);
				add_to_db(&root, &f.hash, &contents);
			}
		}

		let commit_name = serialize_commit(&root, &commit);
		commits_status.current_commit = commit_name.clone();
		commits_status.head_commit = commit_name;

		commits_status.save(&root);
	} else {
		println!("You must be in head commitd to make a new commit.");
	} 
}

