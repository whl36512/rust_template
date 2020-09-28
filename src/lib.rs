pub mod mod1; // look for file name ./mod1.rs
// This module is private, meaning that no external crate can access this
// module. Because it is private at the root of this current crate, however, any
// module in the crate may access any publicly visible item in this module.
// A module can see private sibling module, but not private child module of sibling module.
mod mod2; // look for file name ./mod2.rs    . private module
pub mod mod3; // look for file name ./mod3.rs

mod mod4 { //private module
}

pub mod mod5 {// public module

	#[cfg(test)]
	mod tests {
    		#[test]
    		fn it_works() {
       		 	assert_eq!(2 + 2, 4);
    		}
	}
}
