extern crate walkdir;
use self::walkdir::WalkDir;
use std::path::PathBuf;

use std::ops::Deref;
use ::dir_ops::*;
use std::fs::{create_dir, remove_file};

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
			let path_buf = PathBuf::from(&f.path);
			let path = path_buf.as_path();
			
			if f.hash == "" && !path.is_dir() {
				create_dir(&path).expect("Could not create directory!");
			} else {
				let path_str = String::from(path.to_str().unwrap());
				let content = get_from_db(&path_str, &f.hash);
				write_to_file(&path_buf, &content);
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

		loop {
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