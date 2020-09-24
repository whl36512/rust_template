mod foo;

pub use foo::A ; // re-export private foo::A so it can be used as crate::mod3::A


struct B ;  //will bereference as crate::mod3::B

mod baz {
//private module
// will be reference as crate::mod3::baz::*
}

pub mod bat{
	//public  module
}


