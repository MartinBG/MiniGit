//arguments: []
// remove changes which are not commited

//get current dir
//get head commit
//get differences
//apply
//reverse checkout

use ::dir_ops::*;
use ::commits_status::*;
use ::commit_node::*;
use std::ops::Deref;


pub fn revert(start_path: &String) {
	let root = get_root_dir(start_path);

	let mut commits_status = CommitsStatus::load(&root);
	let current_commit = DeserializeCommit(&root, &commits_status.current_commit);

	let current_state = DirTree::new(&root);

	if current_state == current_commit.dir_state {
		println!("No changes found!");
	} else {
		let differences = current_state.differences(&current_commit.dir_state);
		differences.apply();
	}
}