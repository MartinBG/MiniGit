//arguments: commitId (will be hash of commit)
// change directory as specified in commit info
// cannot commit if not head

//steps:
//1)load commits: destination and current
	// -deserialize commit
	// -get head commit path
//2)Get differences
// 3) Add and delete files

use ::dir_ops::*;
use ::commits_status::*;
use ::commit_node::*;
use std::ops::Deref;


pub fn checkout(start_path: &String, commit_id: &String) {
	let root = get_root_dir(start_path);

	let mut commits_status = CommitsStatus::load(&root);
	if commits_status.current_commit == *commit_id {
		println!("You are already on this commit.");
	} else {
		let current_commit = DeserializeCommit(&root, &commits_status.current_commit);

		let current_state = DirTree::new(&root);

		if current_state != current_commit.dir_state {
			println!("Not commited changes in working directory. Please commit or revert them before yoy checkout.");
		} else {
			let new_commit = DeserializeCommit(&root, commit_id);

			let differences = current_state.differences(&new_commit.dir_state);
			differences.apply();
			commits_status.current_commit = String::from(commit_id.deref());
			commits_status.save(&root); 
		}	
	}
}