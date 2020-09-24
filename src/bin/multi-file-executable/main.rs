//extern crate are not used any more
use rust_template as a ;  // main.rs and lib.rs belong to different crate even though they are in the same directory

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> Result<()> {
	println!("Hello");
	Ok(())
}
