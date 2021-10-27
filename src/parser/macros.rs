use crate::parser::{Attribute, Edge, EdgeTy, GraphAttributes, Id, Node, Port, Subgraph};

#[macro_export]
macro_rules! port {
    () => {None};
    ($id:expr, $str:expr) => {Some(Port($id,$str))}
}

#[macro_export]
macro_rules! id {
    () => { Id::Anonymous("".to_string()) };
    (<<$e:expr,>>) => { Id::Html(format!("<<{}>>",$e))};
    (|$e:expr) => { Id::Escaped(format!("{}",$e))};
    ($e:expr) => { Id::Plain(format!("{}",$e))};
    ($id:expr,$p:expr) => { Id::IdwPort(Box::new($id),$p)};
}


mod tests {
    use crate::parser::{Attribute, Edge, EdgeTy, GraphAttributes, Id, Node, Port, Subgraph};

    #[test]
    fn id_test() {
        assert_eq!(id!(), Id::Anonymous("".to_string()));
        assert_eq!(id!(<<"abc",>>), Id::Html("<<abc>>".to_string()));
        assert_eq!(id!("abc"), Id::Plain("abc".to_string()));
        assert_eq!(id!(|"\"ab\\\"c\""), Id::Escaped("\"ab\\\"c\"".to_string()));
        assert_eq!(id!(id!("abc"),port!(None, Some("n".to_string()))),
                   Id::IdwPort(box Id::Plain("abc".to_string()),
                               Some(Port(None,
                                         Some("n".to_string())))));
    }
}