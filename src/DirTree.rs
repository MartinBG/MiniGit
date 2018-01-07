extern crate walkdir;
use self::walkdir::WalkDir;
use std::path::{Path, PathBuf};
// use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::hash::Hash;
use std::collections::hash_map::DefaultHasher;

extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;

#[derive(Debug)]
struct DirItem {
	path: String,
	hash: String
}

#[derive(Debug)]
pub struct DirTree {
	dir_items: Vec<DirItem>
}

impl DirTree {
	pub fn new(root_path: &String) -> Self {
		let mut dirItems: Vec<DirItem> = vec!();
		for entry in WalkDir::new(root_path).max_depth(1) {
				let path_str = String::from(entry.unwrap().path().to_str().unwrap()); 
				dirItems.push(DirItem {
					path: path_str.clone(),
					hash: gen_hash(&path_str)
				});
			}

		DirTree { 
			dir_items: dirItems
		}
	}
}

fn gen_hash(file_path: &String) -> String {
	let path = Path::new(file_path);
	if path.is_dir() {
		String::new()
	} 
	else {
		let mut f = File::open(&file_path).expect("file not found");

	    let mut contents = String::new();
	    f.read_to_string(&mut contents)
	        .expect("something went wrong reading the file");

	    let mut sha = Sha256::new();
	    sha.input_str(contents.as_ref());
	    sha.result_str()
	}
}

fn add_to_db(file_path: &String) {

}

fn get_root_dir(start_path: &String) -> &String {

}
