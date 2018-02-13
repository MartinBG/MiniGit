extern crate walkdir;
use self::walkdir::WalkDir;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::prelude::*;

extern crate crypto;
use self::crypto::digest::Digest;
use self::crypto::sha1::Sha1;
use ::{serde_json,serde_derive};

#[derive(Serialize, Deserialize, Debug)]
struct DirItem {
	path: String,
	hash: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DirTree {
	dir_items: Vec<DirItem>
}

impl DirTree {
	pub fn new(root_path: &String) -> Self {
		let mut dir_items: Vec<DirItem> = vec!();
		for entry in WalkDir::new(root_path).max_depth(1) {
				
				let walk_dir = match entry {
					Ok(walk_dir) => walk_dir,
					Err(err) => panic!(err) 
				};

				let path = walk_dir.path();
				// println!("{:?}", path);
				let is_dir = path.is_dir();
				let path_str = String::from(path.to_str().unwrap()); 
				
				let mut file_hash = String::new();

				if !is_dir {
					let file_contents = get_file_content(&path.to_path_buf());
					file_hash = gen_hash(&file_contents);
					add_to_db(root_path, &file_hash, &file_contents);
				}

				let new_item = DirItem {
					path: path_str,
					hash: file_hash
				};
				dir_items.push(new_item);
			}

		DirTree { 
			dir_items: dir_items
		}
	}
}

pub fn get_file_content(file_path: &PathBuf) -> String {
	let path = file_path.as_path();
	if path.is_dir() {
		String::new()
	} 
	else {
		println!("before open file");
		let mut f = File::open(&file_path).expect("file not found");
		println!("after open");
	    let mut contents = String::new();
	    f.read_to_string(&mut contents)
	        .expect("something went wrong reading the file");
	    println!("after read");
	    println!("{:?}", contents);
	    return String::from(contents);
	}
}

pub fn write_to_file(file_path: &PathBuf, contents: &String) {
	let mut f = File::create(&file_path).expect("could not create file");

    f.write_all(contents.as_bytes())
        .expect("something went wrong writing the file");
}

fn gen_hash(file_contents: &String) -> String {
	let mut sha = Sha1::new();
    sha.input_str(file_contents.as_ref());
    sha.result_str()
}

fn add_to_db(start_path: &String, file_hash: &String, file_contents: &String) {
	let root_dir = get_root_dir(start_path);

	if *file_hash != String::new() {
		let mut db_path = PathBuf::from(root_dir);
		db_path.push("_init_");
		db_path.push("db");
		db_path.push(file_hash);
		if !db_path.as_path().exists() {
			// let mut f = File::create(&db_path).expect("file cannot open");

			// f.write_all(file_contents.as_bytes());
			write_to_file(&PathBuf::from(db_path), &file_contents)
		}
	}
}

pub fn get_root_dir(start_path: &String) -> String {
	let curr_path = Path::new(start_path);
	let mut curr_path_buff = curr_path.to_path_buf();
	loop {
		curr_path_buff.push("_init_");
		if curr_path_buff.as_path().exists() {
			curr_path_buff.pop();
			
			return String::from(curr_path_buff.as_path().to_str().unwrap());
		}
		else {
			curr_path_buff.pop();
			if !curr_path_buff.pop() {
				panic!("Cannot find repository root directory. Folder _init_ should exists.")
			}
		}
	}
}
