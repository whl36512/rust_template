//extern crate are not used any more
use crate as a ;  // main.rs and lib.rs belong to different crate even though they are in the same directory

//type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

