mod generate;

use crate::{generate_attr, as_item};
use into_attr::IntoAttribute;
use into_attr_derive::IntoAttribute;
use dot_generator::{attr, id};
use dot_structures::*;

struct NodeAttributes {}
struct EdgeAttributes {}
struct GraphAttributes {}
struct SubgraphAttributes {}

generate_attr!(enum shape for NodeAttributes; box_, polygon, ellipse );
generate_attr!(struct center for GraphAttributes; bool );
generate_attr!(struct area for NodeAttributes, SubgraphAttributes; f32; 1.0);
generate_attr!(enum clusterrank for GraphAttributes; local,global,none; local);


#[cfg(test)]
pub mod tests {
    use crate::dot_lang::attributes::*;
    use dot_generator::{attr};
    use crate::dot_lang::attributes::*;
    use into_attr::IntoAttribute;

    #[test]
    fn shape_test() {
        assert_eq!(attr!("shape","box"), shape::box_.into_attr());
        assert_eq!(attr!("center","true"), center(true).into_attr());
        assert_eq!(attr!("area","1"), area::default().into_attr());
        assert_eq!(attr!("clusterrank","local"), clusterrank::default().into_attr());


        assert_eq!(attr!("shape","box"), NodeAttributes::shape(shape::box_));
        assert_eq!(attr!("center","false"), GraphAttributes::center(false));
        assert_eq!(attr!("area","2"), NodeAttributes::area(2.0));
        assert_eq!(attr!("area","2"), SubgraphAttributes::area(2.0));

    }
}