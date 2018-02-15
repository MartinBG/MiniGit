//arguments: []
// shows changed/deleted/added files

// get current dir
// get head commit dir
// get differences 
// print

use ::dir_ops::*;
use ::commits_status::*;
use ::commit_node::*;

pub fn status(start_path: &String) {
	let root = get_root_dir(start_path);

	let commits_status = CommitsStatus::load(&root);
	let current_commit = DeserializeCommit(&start_path, &commits_status.current_commit);

	let current_dir_tree = DirTree::new(&root);

	let diff = current_commit.dir_state.differences(&current_dir_tree);
	
	println!("\nCommit message: {:?}", &current_commit.message);
	println!("Commited on: {:?}", &current_commit.commit_time);
	diff.print();
}