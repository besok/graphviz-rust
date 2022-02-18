//! # The set of macroses helping to generate the elements of graphviz
//! The set helps to generate the major components of the graphviz dot notation
//! endevouring to follow comparatively close to the language [`notation`]
//!
//! [`notation`]: https://graphviz.org/doc/info/lang.html
//! # Description:
//! In overall, the format of macros is the following one:
//!  - name or id or any other markers
//!  - list of vec with a prefix , or seq of elems with a prefix ;
//!
//! #Note:
//!  - for the list of items the way to pass vec is the following one: element(.. , vec of items)
//!  - for the seq of items the way to pass several items is the following one: element(.. ; items+)
//!
//! # Examples:
//! ```rust
//!        fn graph_test() {
//!        use dot_generator::*;
//!        use dot_structures::*;
//!
//!        let g = r#"
//!        strict digraph t {
//!            aa[color=green]
//!            subgraph v {
//!                aa[shape=square]
//!                subgraph vv{a2 -> b2}
//!                aaa[color=red]
//!                aaa -> bbb
//!            }
//!            aa -> be -> subgraph v { d -> aaa}
//!            aa -> aaa -> v
//!        }
//!        "#;
//!
//!            graph!(strict di id!("t");
//!              node!("aa";attr!("color","green")),
//!              subgraph!("v";
//!                node!("aa"; attr!("shape","square")),
//!                subgraph!("vv"; edge!(node_id!("a2") => node_id!("b2"))),
//!                node!("aaa";attr!("color","red")),
//!                edge!(node_id!("aaa") => node_id!("bbb"))
//!                ),
//!              edge!(node_id!("aa") => node_id!("be") => subgraph!("v"; edge!(node_id!("d") => node_id!("aaa")))),
//!              edge!(node_id!("aa") => node_id!("aaa") => node_id!("v"))
//!            );
//!    }
//! ```
use dot_structures::*;

/// represents a port in dot lang
#[macro_export]
macro_rules! port {
    () => {Port(None,None)};
    ( , $str:expr) => { Port(None,Some($str.to_string()))};
    ( $id:expr , $str:expr) => {Port(Some($id),Some($str.to_string()))};
    ( $id:expr) => {Port(Some($id),None)};
}
/// represents a node id in dot lang
/// Essentially it is a combination of id and port
#[macro_export]
macro_rules! node_id {
    () => {  NodeId(id!(),None) };
    ($e:expr) => { NodeId(id!($e),None) };
    ($e:expr, $p:expr) => { NodeId(id!($e),Some($p)) };
    ($i:ident $e:expr) => { NodeId(id!($i$e),None) };
    ($i:ident $e:expr, $p:expr) => { NodeId(id!($i$e),Some($p)) };
}

/// represents an id for node or subgraph in dot lang.
/// #Arguments:
///  - html - html format.
///  - esc - escaped string. It allows the escaped quotes inside and also wraps the string to the quotas
/// #Example:
/// ```rust
///     fn id_test() {
///         use dot_generator::*;
///         use dot_structures::*;
///
///         assert_eq!(id!(), Id::Anonymous("".to_string()));
///         assert_eq!(id!(html "<<abc>>"), Id::Html("<<abc>>".to_string()));
///         assert_eq!(id!("abc"), Id::Plain("abc".to_string()));
///         assert_eq!(id!(esc r#"ab\"c"#"), Id::Escaped(r#"\"ab\"c\""#.to_string()));
///     }
/// ```
#[macro_export]
macro_rules! id {
    () => { Id::Anonymous("".to_string()) };
    (html$e:expr) => { Id::Html(format!("{}",$e))};
    (esc$e:expr) => { Id::Escaped(format!("\"{}\"",$e))};
    ($e:expr) => { Id::Plain(format!("{}",$e))};
}

/// represents an attribute in dot lang.
/// # Example:
/// ```rust
///     fn attr_test() {
///         use dot_generator::*;
///         use dot_structures::*;
///
///         assert_eq!(attr!("a","1"), Attribute(id!("a"), id!("1")));
///         assert_eq!(attr!(html "a","1"), Attribute(id!(html "a"), id!("1")))
///     }
/// ```
#[macro_export]
macro_rules! attr {
    ($ik:ident $k:expr,$iv:ident $v:expr) => {Attribute(id!($k),id!($iv $v))};
    ($ik:ident $k:expr,$v:expr) => {Attribute(id!($ik $k),id!($v))};
    ($k:expr, $iv:ident $v:expr) => {Attribute(id!($k),id!($iv $v))};
    ($k:expr,$v:expr) => {Attribute(id!($k),id!($v))}
}

/// represents an element of graph or subgraph which is, in turn, just a wrapper
/// for the underlying structure as node,edge, subgraph etc.
/// #Example:
/// ```rust
///     fn stmt_test() {
///         use dot_generator::*;
///         use dot_structures::*;
///
///         assert_eq!(stmt!(node!()), Stmt::Node(Node::new(NodeId(id!(), None), vec![])));
///     }
/// ```
#[macro_export]
macro_rules! stmt {
    ($k:expr) => {Stmt::from($k)};
}

/// represents a subgraph in dot lang.
/// #Example:
/// ```rust
///     fn subgraph_test() {
///         use dot_generator::*;
///         use dot_structures::*;
///
///         assert_eq!(subgraph!(), Subgraph { id: Id::Anonymous("".to_string()), stmts: vec![] });
///         assert_eq!(subgraph!("abc";node!()),
///                    Subgraph {
///                        id: Id::Plain("abc".to_string()),
///                        stmts: vec![stmt!(node!())],
///                    });
///     }
/// ```
#[macro_export]
macro_rules! subgraph {
    () => {Subgraph{id:id!(),stmts:vec![]}};
    ($id:expr) => {Subgraph{id:id!($id),stmts:vec![]}};
    ($i:ident $id:expr) => {Subgraph{id:id!($i$id),stmts:vec![]}};
    ($id:expr, $stmts:expr) => {Subgraph{id:id!($id),stmts:$stmts}};
    ($i:ident $id:expr, $stmts:expr) => {Subgraph{id:id!($i$id),stmts:$stmts}};
    ($i:ident $id:expr; $($stmts:expr),+ ) => {{
        let mut stmts_vec = Vec::new();
        $( stmts_vec.push(stmt!($stmts)) ; )+
        Subgraph{id:id!($i$id),stmts:stmts_vec}
    }};
    ($id:expr; $($stmts:expr),+ ) => {{
        let mut stmts_vec = Vec::new();
        $( stmts_vec.push(stmt!($stmts)) ; )+
        Subgraph{id:id!($id),stmts:stmts_vec}
    }};
    (; $($stmts:expr),+ ) => {{
        let mut stmts_vec = Vec::new();
        $( stmts_vec.push(stmt!($stmts)) ; )+
        Subgraph{id:id!(),stmts:stmts_vec}
    }};

}

/// represents a node in dot lang.
/// #Example:
/// ```rust
///     fn node_test() {
///         use dot_generator::*;
///         use dot_structures::*;
///
///         assert_eq!(node!(), Node::new(NodeId(id!(), None), vec![]));
///         assert_eq!(node!(html "abc"; attr!("a","a")),
///                    Node::new(NodeId(id!(html "abc"), None),
///                              vec![attr!("a","a")]));
///         assert_eq!(node!(html "abc" ; attr!("a","a")),
///                    Node::new(NodeId(id!(html "abc"), None),
///                              vec![attr!("a","a")]));
///         assert_eq!(node!("abc" ; attr!("a","a"),attr!("a","a")),
///                    Node::new(NodeId(id!( "abc"), None),
///                              vec![attr!("a","a"), attr!("a","a")]))
///     }
/// ```
#[macro_export]
macro_rules! node {
    () => {Node::new(NodeId(id!(), None), vec![])};
    ($i:ident $id:expr) => {Node::new(NodeId(id!($i$id), None), vec![])};
    ($id:expr) => {Node::new(NodeId(id!($id), None), vec![])};
    ($i:ident $id:expr; $($attr:expr),+ ) => {{
        let mut attrs = Vec::new();
        $( attrs.push($attr) ; )+
        Node::new(NodeId(id!($i$id), None), attrs)
    }};
     ($i:ident $id:expr, $attrs:expr  ) => {
        Node::new(NodeId(id!($i$id), None), $attrs)
    };
    ($id:expr, $attrs:expr  ) => {
        Node::new(NodeId(id!($id), None), $attrs)
    };
    ( $id:expr; $($attr:expr),+ ) => {{
        let mut attrs = Vec::new();
        $( attrs.push($attr) ; )+
        Node::new(NodeId(id!( $id), None), attrs)
    }};
    ($i:ident $id:expr => $p:expr, $attrs:expr  ) => {
        Node::new(NodeId(id!($i$id), Some($p)), $attrs)
    };
    ($i:ident $id:expr => $p:expr; $($attr:expr),+ ) => {{
        let mut attrs = Vec::new();
         $( attrs.push($attr) ; )+
        Node::new(NodeId(id!($i$id), Some($p)), attrs)
    }};
    ( $id:expr => $p:expr, $attrs:expr  ) => {
        Node::new(NodeId(id!($id), Some($p)), $attrs)
    };
    ( $id:expr => $p:expr; $($attr:expr),+ ) => {{
        let mut attrs = Vec::new();
        $( attrs.push($attr) ; )+
        Node::new(NodeId(id!($id), Some($p)), attrs)
    }};
}

/// represents an edge in dot lang.
/// #Example:
/// ```rust
///     fn edge_test() {
///         use dot_generator::*;
///         use dot_structures::*;
///
///          assert_eq!(
///             edge!(node_id!("1") => node_id!("2")),
///             Edge { ty: EdgeTy::Pair(Vertex::N(node_id!("1")), Vertex::N(node_id!("2"))), attributes: vec![] }
///         );
///         assert_eq!(
///             edge!(node_id!("1") => node_id!("2") => subgraph!("a")),
///             Edge { ty: EdgeTy::Chain(vec![Vertex::N(node_id!("1")), Vertex::N(node_id!("2")), Vertex::S(subgraph!("a"))]), attributes: vec![] }
///         );
///         assert_eq!(
///             edge!(node_id!("1") => node_id!("2"), vec![attr!("a","b")]),
///             Edge { ty: EdgeTy::Pair(Vertex::N(node_id!("1")), Vertex::N(node_id!("2"))), attributes: vec![attr!("a","b")] }
///         );
///         assert_eq!(
///             edge!(node_id!("1") => node_id!("2"); attr!("a","b")),
///             Edge { ty: EdgeTy::Pair(Vertex::N(node_id!("1")), Vertex::N(node_id!("2"))), attributes: vec![attr!("a","b")] }
///         );
///     }
/// ```
#[macro_export]
macro_rules! edge {
    ($l:expr => $r:expr) => {
        Edge{ ty: EdgeTy::Pair(Vertex::from($l),Vertex::from($r)), attributes: vec![] }
    };
    ($l:expr => $r:expr $(=> $nexts:expr)+) => {{
         let mut edges_vec = vec![Vertex::from($l),Vertex::from($r)];
         $( edges_vec.push(Vertex::from($nexts)) ; )+

        Edge{ ty: EdgeTy::Chain(edges_vec), attributes: vec![] }
    }};

    ($l:expr => $r:expr, $attrs:expr) => {
        Edge{ ty: EdgeTy::Pair(Vertex::from($l),Vertex::from($r)), attributes: $attrs };
    };
    ($l:expr => $r:expr; $($attrs:expr),+) => {{
         let mut attrs_vec = Vec::new();
        $( attrs_vec.push($attrs) ; )+
        Edge{ ty: EdgeTy::Pair(Vertex::from($l),Vertex::from($r)), attributes: attrs_vec }
    }};
    ($l:expr => $r:expr $(=> $nexts:expr)+; $($attrs:expr),+) => {{
         let mut attrs_vec = Vec::new();
         $( attrs_vec.push($attrs) ; )+

         let mut edges_vec = vec![Vertex::from($l),Vertex::from($r)];
         $( edges_vec.push(Vertex::from($nexts)) ; )+

        Edge{ ty: EdgeTy::Chain(edges_vec), attributes: attrs_vec }
    }};
    ($l:expr => $r:expr $(=> $nexts:expr)+ , $attrs:expr) => {{

         let mut edges_vec = vec![Vertex::from($l),Vertex::from($r)]
         $( edges_vec.push(Vertex::from($nexts)) ; )+

        Edge{ ty: EdgeTy::Chain(edges_vec), attributes: $attrs }
    }};
}

/// represents a graph in dot lang.
///  - strict word stands for strict in graph
///  - di word stands for digraph
/// #Example:
/// ```rust
///     fn graph_test() {
///         use dot_generator::*;
///         use dot_structures::*;
///
///          assert_eq!(
///             graph!(strict di id!("abc")),
///             Graph::DiGraph { id: id!("abc"), strict: true, stmts: vec![] }
///         );
///         assert_eq!(
///             graph!(strict di id!("abc");node!("abc")),
///             Graph::DiGraph { id: id!("abc"), strict: true, stmts: vec![stmt!(node!("abc"))] }
///         );
///     }
/// ```
#[macro_export]
macro_rules! graph {
    (strict $id:expr) => {
        Graph::Graph { id: $id, strict: true, stmts: vec![] }
    };
    ($id:expr) => {
        Graph::Graph { id: $id, strict: false, stmts: vec![] }
    };
    (strict di $id:expr) => {
        Graph::DiGraph { id: id!($id), strict: true, stmts: vec![] }
    };
    (di $id:expr) => {
        Graph::DiGraph { id: id!($id), strict: false, stmts: vec![] }
    };
    (strict $id:expr, $stmts:expr) => {
        Graph::Graph { id: $id, strict: true, stmts: $stmts }
    };
    ($id:expr, $stmts:expr) => {
        Graph::Graph { id: $id, strict: false, stmts: $stmts }
    };
    (strict di $id:expr, $stmts:expr) => {
        Graph::DiGraph { id: $id, strict: true, stmts: $stmts }
    };
    (di $id:expr, $stmts:expr) => {
        Graph::DiGraph { id: $id, strict: false, stmts: $stmts }
    };

    (strict $id:expr; $($stmts:expr),+) => {{
         let mut stmts = vec![];
         $( stmts.push(stmt!($stmts)) ; )+
        Graph::Graph { id: $id, strict: true, stmts: stmts }
    }};
    ($id:expr; $($stmts:expr),+) => {{
         let mut stmts = vec![];
         $( stmts.push(stmt!($stmts)) ; )+
        Graph::Graph { id: $id, strict: false, stmts: stmts }
    }};
    (strict di $id:expr; $($stmts:expr),+) => {{
         let mut stmts = vec![];
         $( stmts.push(stmt!($stmts)) ; )+
         Graph::DiGraph { id: $id, strict: true, stmts: stmts }
    }};
    (di $id:expr; $($stmts:expr),+) => {{
         let mut stmts = vec![];
         $( stmts.push(stmt!($stmts)) ; )+
         Graph::DiGraph { id: $id, strict: false, stmts: stmts }
    }};

}

#[cfg(test)]
mod tests {
    use dot_structures::*;

    #[test]
    fn graph_test() {
        assert_eq!(
            graph!(strict di id!("abc")),
            Graph::DiGraph { id: id!("abc"), strict: true, stmts: vec![] }
        );
        assert_eq!(
            graph!(strict di id!("abc");stmt!(node!("abc"))),
            Graph::DiGraph { id: id!("abc"), strict: true, stmts: vec![stmt!(node!("abc"))] }
        );
    }

    #[test]
    fn edge_test() {
        assert_eq!(
            edge!(node_id!("1") => node_id!("2")),
            Edge { ty: EdgeTy::Pair(Vertex::N(node_id!("1")), Vertex::N(node_id!("2"))), attributes: vec![] }
        );
        assert_eq!(
            edge!(node_id!("1") => node_id!("2") => subgraph!("a")),
            Edge { ty: EdgeTy::Chain(vec![Vertex::N(node_id!("1")), Vertex::N(node_id!("2")), Vertex::S(subgraph!("a"))]), attributes: vec![] }
        );
        assert_eq!(
            edge!(node_id!("1") => node_id!("2"), vec![attr!("a","b")]),
            Edge { ty: EdgeTy::Pair(Vertex::N(node_id!("1")), Vertex::N(node_id!("2"))), attributes: vec![attr!("a","b")] }
        );
        assert_eq!(
            edge!(node_id!("1") => node_id!("2"); attr!("a","b")),
            Edge { ty: EdgeTy::Pair(Vertex::N(node_id!("1")), Vertex::N(node_id!("2"))), attributes: vec![attr!("a","b")] }
        );
    }

    #[test]
    fn stmt_test() {
        assert_eq!(stmt!(node!()), Stmt::Node(Node::new(NodeId(id!(), None), vec![])));
    }

    #[test]
    fn subgraph_test() {
        assert_eq!(subgraph!(), Subgraph { id: Id::Anonymous("".to_string()), stmts: vec![] });
        assert_eq!(subgraph!("abc";node!()),
                   Subgraph {
                       id: Id::Plain("abc".to_string()),
                       stmts: vec![stmt!(node!())],
                   });
    }

    #[test]
    fn node_test() {
        assert_eq!(node!(), Node::new(NodeId(id!(), None), vec![]));
        assert_eq!(node!(html "abc"; attr!("a","a")),
                   Node::new(NodeId(id!(html "abc"), None),
                             vec![attr!("a","a")]));
        assert_eq!(node!(html "abc" ; attr!("a","a")),
                   Node::new(NodeId(id!(html "abc"), None),
                             vec![attr!("a","a")]));
        assert_eq!(node!("abc" ; attr!("a","a"),attr!("a","a")),
                   Node::new(NodeId(id!( "abc"), None),
                             vec![attr!("a","a"), attr!("a","a")]))
    }

    #[test]
    fn attr_test() {
        assert_eq!(attr!("a","1"), Attribute(id!("a"), id!("1")));
        assert_eq!(attr!(html "a","1"), Attribute(id!(html "a"), id!("1")))
    }

    #[test]
    fn id_test() {
        assert_eq!(id!(), Id::Anonymous("".to_string()));
        assert_eq!(id!(html "<<abc>>"), Id::Html("<<abc>>".to_string()));
        assert_eq!(id!("abc"), Id::Plain("abc".to_string()));
        assert_eq!(id!(esc "ab\\\"c"), Id::Escaped("\"ab\\\"c\"".to_string()));
    }
}
