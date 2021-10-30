mod macros;

use crate::{gen_attr, as_item};
use into_attr::IntoAttribute;
use into_attr_derive::IntoAttribute;
use dot_generator::{attr, id};
use dot_structures::*;


gen_attr!(shape; box_, polygon, ellipse );

#[cfg(test)]
pub mod tests {
    use crate::dot_lang::attributes::shape;
    use dot_generator::{attr };
    use crate::dot_lang::attributes::*;

    #[test]
    fn shape_test() {
        assert_eq!(attr!("shape","box"), shape::box_.into_attr());
    }
}