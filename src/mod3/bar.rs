//will be reference as crate::mod3::bar::*
use super::foo;
// or use crate::utils::foo;

pub fn bar() {
    foo::say_foo();
}
