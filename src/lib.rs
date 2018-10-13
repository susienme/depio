//! Theseus module dependency IO wrapper
//!
//! DepWriter is supposed to used by the compiler
//! read_from_mem() is supposed to used by the kernel.
//!
//! #Example
//! ```
//! let mem: &str;
//! 
//! // Make mem point to serialized dep data
//!
//! let deps = depio::read_from_mem(mem);
//! ```

#![no_std]
#![feature(alloc)]

extern crate alloc;
#[macro_use]
extern crate serde_json;
#[macro_use]

use alloc::string::String;
use alloc::btree_map::BTreeMap;
use alloc::btree_set::BTreeSet;

pub struct DepWriter {
	deps: BTreeMap<String, BTreeSet<String>>,
}

impl DepWriter {
	pub fn new() -> DepWriter {
		DepWriter {
			deps: BTreeMap::new()
		}
	}

	pub fn write(&mut self, dependent: String, to_whom: BTreeSet<String>) {
		self.deps.insert(dependent, to_whom);
	}

	pub fn finish(&self) -> String {
		serde_json::to_string(&self.deps).unwrap()
	}
}

pub fn read_from_mem(data: &str) -> Option<BTreeMap<String, BTreeSet<String>>> {
	match serde_json::from_str(data) {
		Ok(rtn) => {return rtn;}
		_ => {return None;}
	}
}