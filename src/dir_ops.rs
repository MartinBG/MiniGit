use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::prelude::*;

extern crate crypto;
use self::crypto::digest::Digest;
use self::crypto::sha1::Sha1;

pub fn get_file_content(file_path: &PathBuf) -> String {
	let path = file_path.as_path();
	if path.is_dir() {
		String::new()
	} 
	else {
		let mut f = File::open(&file_path).expect("file not found");
	    let mut contents = String::new();
	    f.read_to_string(&mut contents)
	        .expect("something went wrong reading the file");
	    return String::from(contents);
	}
}

pub fn write_to_file(file_path: &PathBuf, contents: &String) {
	let mut f = File::create(&file_path).expect("could not create file");

    f.write_all(contents.as_bytes())
        .expect("something went wrong writing the file");
}

pub fn gen_hash(file_contents: &String) -> String {
	let mut sha = Sha1::new();
    sha.input_str(file_contents.as_ref());
    sha.result_str()
}

pub fn add_to_db(start_path: &String, file_hash: &String, file_contents: &String) {
	let root_dir = get_root_dir(start_path);

	if *file_hash != String::new() {
		let mut db_path = PathBuf::from(root_dir);
		db_path.push("_init_");
		db_path.push("db");
		db_path.push(file_hash);
		if !db_path.as_path().exists() {
			write_to_file(&PathBuf::from(db_path), &file_contents)
		}
	}
}

pub fn get_from_db(start_path: &String, hash: &String) -> String {
	let root_dir = get_root_dir(start_path);

	let mut db_path = PathBuf::from(root_dir);
	db_path.push("_init_");
	db_path.push("db");
	db_path.push(hash);
	return get_file_content(&db_path);
}

pub fn exists_in_db(start_path: &String, path: &String) -> bool {
	let root_dir = get_root_dir(start_path);

	let mut db_path = PathBuf::from(root_dir);
	db_path.push("_init_");
	db_path.push("db");
	db_path.push(path);
	let path = db_path.as_path();

	path.is_file()
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
