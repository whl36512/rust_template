pub mod mod1; // look for file name ./mod1
mod mod2; // look for file name ./mod2    . private module
pub mod mod3; // look for file name ./mod3

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
