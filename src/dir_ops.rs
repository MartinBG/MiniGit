extern crate walkdir;
use self::walkdir::WalkDir;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::prelude::*;

extern crate crypto;
use self::crypto::digest::Digest;
use self::crypto::sha1::Sha1;
use ::{serde_json,serde_derive};

use std::ops::Deref;
use std::fs::{remove_file, create_dir};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DirItem {
	pub path: String,
	pub hash: String
}

impl DirItem {
	pub fn new() -> Self {
		DirItem {
			path: String::new(),
			hash: String::new()
		}
	}
}

#[derive(Debug)]
pub struct DirTreeDifferences {
	removed_items: Vec<DirItem>,
	added_items: Vec<DirItem>
}

impl DirTreeDifferences {
	pub fn new() -> Self {
		DirTreeDifferences {
			removed_items: vec!(),
			added_items: vec!()
		}
	}

	pub fn apply(&self) {
		for f in self.removed_items.iter() {
			if f.hash != "" {
				let path = PathBuf::from(&f.path);
				remove_file(&path).expect("Could not delete file!");
			}
		}
		for f in self.added_items.iter() {
			let pathBuf = PathBuf::from(&f.path);
			let path = pathBuf.as_path();
			
			if f.hash == "" && !path.is_dir() {
				create_dir(&path).expect("Could not create directory!");
			} else {
				let path_str = String::from(path.to_str().unwrap());
				let content = get_from_db(&path_str, &f.hash);
				write_to_file(&pathBuf, &content);
			}
		}
	}


	pub fn print(&self) {
		println!();
		if self.added_items.len() == 0 && self.removed_items.len() == 0 {
			print!("No changes in directory!");
		} else {
			for x in self.added_items.iter() {
				if x.hash != "" {
					println!("++{:?}", x.path);
				}
			}
			println!("\n");
			for x in self.removed_items.iter() {
				if x.hash != "" {
					println!("--{:?}", x.path);
				}
			}
		}
	}
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DirTree {
	pub dir_items: Vec<DirItem>
}


impl DirTree {
	pub fn new(root_path: &String) -> Self {
		let mut dir_items: Vec<DirItem> = vec!();
		for entry in WalkDir::new(root_path) {
				
				let walk_dir = match entry {
					Ok(walk_dir) => walk_dir,
					Err(err) => panic!(err) 
				};

				let path = walk_dir.path();
				let is_dir = path.is_dir();
				let path_str = String::from(path.to_str().unwrap()); 
				
				if path_str.contains("_init_") {
					continue;
				}

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

	pub fn differences(&self, other: &DirTree) -> DirTreeDifferences {
		let mut diff = DirTreeDifferences::new();

		let mut it = self.dir_items.iter();
		let mut other_it = other.dir_items.iter();
		
		let mut it_stop = false;
		let mut other_it_stop = false;

		let dummy = DirItem::new();

		let mut it_el: &DirItem = match it.next() {
				Some(x) => x,
				None => {it_stop = true; &dummy}
			};
		let mut other_it_el: &DirItem = match other_it.next() {
				Some(x) => x,
				None => {other_it_stop = true;  &dummy}
			};

		let mut cnt = 0;
		loop {
			cnt+=1;
			if it_stop && other_it_stop {
				return diff;
			}
			else if it_stop || (!other_it_stop && it_el.path > other_it_el.path) {
				diff.added_items.push(other_it_el.deref().clone());
				other_it_el = match other_it.next() {
					Some(x) => x,
					None => {other_it_stop = true;  &dummy}
				};
			}
			else if other_it_stop || (!it_stop && it_el.path < other_it_el.path) {
				diff.removed_items.push(it_el.clone());
				it_el = match it.next() {
					Some(x) => x,
					None => {it_stop = true; &dummy}
				};
			}
			else if it_el.path == other_it_el.path {
				if it_el.hash != other_it_el.hash {
					diff.added_items.push(other_it_el.deref().clone());
					diff.removed_items.push(it_el.deref().clone());
				}
				it_el = match it.next() {
					Some(x) => x,
					None => {it_stop = true; &dummy}
				};
				other_it_el = match other_it.next() {
					Some(x) => x,
					None => {other_it_stop = true;  &dummy}
				};
			}
		}
	}

	pub fn print(&self) {
		for x in self.dir_items.iter() {
			println!("{:?}", x);
		}
	}
}

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
			// let mut f = File::create(&db_path).expect("file cannot open");

			// f.write_all(file_contents.as_bytes());
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
