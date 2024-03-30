//! # The set of macroses helping to generate the elements of graphviz
//! The set helps to generate the major components of the graphviz dot notation
//! endeavouring to follow comparatively close to the language [`notation`]
//!
//! [`notation`]: https://graphviz.org/doc/info/lang.html
//! # Description:
//! In overall, the format of macros is the following one:
//!  - name or id or any other markers
//!  - list of vec with a prefix , or seq of elems with a prefix ;
//!
//! # Note:
//!  - for the list of items the way to pass vec is the following one: element(.. , vec of items)
//!  - for the seq of items the way to pass several items is the following one: element(.. ; items+)
//!
//! # Examples:
//! ```rust
//! fn graph_test() {
//!     use dot_generator::*;
//!     use dot_structures::*;
//!
//!     let g = r#"
//!         strict digraph t {
//!             aa[color=green]
//!             subgraph v {
//!                 aa[shape=square]
//!                 subgraph vv{a2 -> b2}
//!                 aaa[color=red]
//!                 aaa -> bbb
//!             }
//!             aa -> be -> subgraph v { d -> aaa}
//!             aa -> aaa -> v
//!         }
//!         "#;
//!
//!     graph!(strict di id!("t");
//!       node!("aa";attr!("color","green")),
//!       subgraph!("v";
//!         node!("aa"; attr!("shape","square")),
//!         subgraph!("vv"; edge!(node_id!("a2") => node_id!("b2"))),
//!         node!("aaa";attr!("color","red")),
//!         edge!(node_id!("aaa") => node_id!("bbb"))
//!         ),
//!       edge!(node_id!("aa") => node_id!("be") => subgraph!("v"; edge!(node_id!("d") => node_id!("aaa")))),
//!       edge!(node_id!("aa") => node_id!("aaa") => node_id!("v"))
//!     );
//! }
//! ```
use dot_structures::*;

/// Constructs a port in dot lang
/// Port consists of two parts: id and directions
/// The both can be optional.
///
/// # Arguments:
///  - id: accepts id of the port. Can be constructed with id!. Can be omitted, like `port!(, "x")`
///  - direction: accepts string. Can be omitted
///
/// # Examples
/// ```rust
///  fn port_test() {
///     use dot_generator::*;
///     use dot_structures::*;
///     assert_eq!(
///         port!(),
///         Port(None,None)
///     );
///     assert_eq!(
///         port!(,""),
///         Port(None,Some("".to_string()))
///     );
///     assert_eq!(
///         port!(id!(),""),
///         Port(Some( Id::Anonymous("".to_string())),Some("".to_string()))
///     );
///     assert_eq!(
///         port!(id!()),
///         Port(Some( Id::Anonymous("".to_string())),None)
///     );
/// }
/// ```
#[macro_export]
macro_rules! port {
    () => {
        Port(None, None)
    };
    (, $str:expr) => {
        Port(None, Some($str.to_string()))
    };
    ($id:expr, $str:expr) => {
        Port(Some($id), Some($str.to_string()))
    };
    ($id:expr) => {
        Port(Some($id), None)
    };
}
/// Constructs a node id in dot lang.
/// In short, it is a pair of id and port.
///
/// # Arguments
/// - id: id of the node, can be omitted. To construct id it accepts
///     - ident (optional): `esc` or `html` prefix
///     - Should be an expression with a result that can be transformed to string (with Display)
/// - port: expects a `Port`. Can be constructed with `port!`
///
/// # Examples
/// ```rust
///     use dot_generator::*;
///     use dot_structures::*;
///
/// fn node_id_test() {
///         assert_eq!(
///             node_id!(),
///             NodeId(Id::Anonymous("".to_owned()), None)
///         );
///
///         assert_eq!(
///             node_id!("plain"),
///             NodeId(Id::Plain("plain".to_owned()), None)
///         );
///
///         assert_eq!(
///             node_id!("plain", port!()),
///             NodeId(Id::Plain("plain".to_owned()), Some(Port(None,None)))
///         );
///
///         assert_eq!(
///             node_id!(esc "escaped"),
///             NodeId(Id::Escaped("\"escaped\"".to_owned()),None)
///         );
///         assert_eq!(
///             node_id!(html "</br>escaped"),
///             NodeId(Id::Html("</br>escaped".to_owned()),None)
///         );
///     }
/// ```

#[macro_export]
macro_rules! node_id {
    () => {  NodeId(id!(),None) };
    ($e:expr) => { NodeId(id!($e),None) };
    ($e:expr, $p:expr) => { NodeId(id!($e),Some($p)) };
    ($i:ident $e:expr) => { NodeId(id!($i$e),None) };
    ($i:ident $e:expr, $p:expr) => { NodeId(id!($i$e),Some($p)) };
}

/// Constructs an id for node or subgraph in dot lang.
/// # Arguments:
///  - prefix (optional):
///     - html: defines the id has html tags in the name
///     - esc: encompasses the given name with quotes and denotes the string has escaped quotes
///     - can be omitted. In that case, the plain id will be created.
///  - Should be an expression with a result that can be transformed to string (with Display)
/// # Examples:
/// ```rust
///     fn id_test() {
///         use dot_generator::*;
///         use dot_structures::*;
///
///         assert_eq!(id!(), Id::Anonymous("".to_string()));
///         assert_eq!(id!(html "<<abc>>"), Id::Html("<<abc>>".to_string()));
///         assert_eq!(id!("abc"), Id::Plain("abc".to_string()));
///         assert_eq!(id!(esc "ab\"c"), Id::Escaped("ab\"c".to_string()));
///     }
/// ```
#[macro_export]
macro_rules! id {
    () => {
        Id::Anonymous("".to_string())
    };
    (html $e:expr) => {
        Id::Html(format!("{}", $e))
    };
    (esc $e:expr) => {
        Id::Escaped(format!("\"{}\"", $e))
    };
    ($e:expr) => {
        Id::Plain(format!("{}", $e))
    };
}

/// Constructs an attribute in dot lang.
/// Essentially it is a pair of key, value separated with `=`
/// This macro composes internal parts of 2 id! macros
///
/// # Arguments
///  - key prefix (optional): esc or html or empty. See `id!` macros
///  - id value for key. Should be an expression with a result that can be transformed to string
///  - value prefix (optional): esc or html or empty. See `id!` macros
///  - id value for value. Should be an expression with a result that can be transformed to string
///
/// # Examples:
/// ```rust
///     fn attr_test() {
///         use dot_generator::*;
///         use dot_structures::*;
///
///         assert_eq!(attr!("a","1"), Attribute(id!("a"), id!("1")));
///         assert_eq!(attr!(html "a","1"), Attribute(id!(html "a"), id!("1")));
///         assert_eq!(attr!(esc "a","1"), Attribute(id!(esc "a"), id!("1")));
///         assert_eq!(attr!(esc "a",esc "1"), Attribute(id!(esc "a"), id!(esc "1")));
///         assert_eq!(attr!("a",esc "1"), Attribute(id!("a"), id!(esc "1")))
///     }
/// ```
#[macro_export]
macro_rules! attr {
    ($ik:ident $k:expr,$iv:ident $v:expr) => {Attribute(id!($k),id!($iv $v))};
    ($ik:ident $k:expr,$v:expr) => {Attribute(id!($ik $k),id!($v))};
    ($k:expr, $iv:ident $v:expr) => {Attribute(id!($k),id!($iv $v))};
    ($k:expr,$v:expr) => {Attribute(id!($k),id!($v))}
}

/// Constructs one of elements of the graph:
///  - Node : node in the graph
///  - Subgraph: subgraph in the graph
///  - Attribute: attribute of an element
///  - GAttribute: attribute for the given graph or subgraph
///  - Edge: edge in the graph
///
/// # Argument:
/// - one of the aforementioned items.
///
/// # Examples:
/// ```rust
/// fn stmt_test() {
///     use dot_generator::*;
///     use dot_structures::*;
///
///     assert_eq!(
///         stmt!(node!(esc "id")), // if the id has escaped string or needs to be placed in the quotes
///         Stmt::Node(Node::new(NodeId(id!(esc "id"), None), vec![]))
///     );
///    assert_eq!(
///         stmt!(attr!("a","1")),
///         Stmt::Attribute(Attribute(id!("a"), id!("1")))
///     );
///    assert_eq!(
///         stmt!(attr!("a",esc "format")),
///         Stmt::Attribute(Attribute(id!("a"), id!(esc "format")))
///     );
///    assert_eq!(
///         stmt!(GraphAttributes::new("graph", vec![attr!("a",esc "format")])),
///         Stmt::GAttribute(GraphAttributes::Graph(vec![attr!("a",esc "format")]))
///     );
///    assert_eq!(
///         stmt!(edge!(node_id!("a") => node_id!("b"))),
///         Stmt::Edge(edge!(node_id!("a") => node_id!("b")))
///     );
/// }
/// ```
#[macro_export]
macro_rules! stmt {
    ($k:expr) => {
        Stmt::from($k)
    };
}

/// Constructs a subgraph in dot lang.
///
/// # Arguments:
/// - prefix for id (optional): esc or html
/// - id: id of the graph. Can be omitted but the separation needs to be left in like `; ...`
/// - a collection of statements or variadic arguments but the delegates from the statement
///
/// Therefore, the macros accepts two different forms where the first separator denotes it:
/// - `subgraph!(id, vec![statement])` where statement can be constructed using the `stmt!` macros
/// - `subgraph!(id; statement_delegate, statement_delegate2, ...)
///     where the delegates are undertaking structures. See `stmt! macros
///
/// # Note:
/// - id can be left empty with varargs like that `subgraph!(; statement1, ...)`
/// - the macros expects the statement delegates, e.g. nodes, edges, attributes. See `stmt!` macro
/// # Examples:
/// ```rust
///     fn subgraph_test() {
///         use dot_generator::*;
///         use dot_structures::*;
///
///         assert_eq!(
///             subgraph!(),
///             Subgraph { id: Id::Anonymous("".to_string()), stmts: vec![] }
///         );
///
///         assert_eq!(
///                   subgraph!(
///                         "abc";
///                         node!("node1"),
///                         node!("node2"),
///                         edge!(node_id!("node1") => node_id!("node2"))
///                   ),
///                   Subgraph {
///                        id: id!("abc"),
///                        stmts: vec![
///                             node!("node1").into(),
///                             node!("node2").into(),
///                             edge!(node_id!("node1") => node_id!("node2")).into()
///                         ],
///                    });
///         assert_eq!(
///                    subgraph!(
///                         "abc",
///                         vec![stmt!(node!("node1")), stmt!(attr!("a","b"))]),
///                    Subgraph {
///                        id: id!("abc"),
///                        stmts: vec![
///                             node!("node1").into(),
///                             attr!("a","b").into()
///                         ],
///                    });
///         assert_eq!(
///                    subgraph!(
///                         esc "abc";
///                         edge!(node_id!("left") => node_id!("right"))),
///                    Subgraph {
///                        id: id!(esc "abc"),
///                        stmts: vec![
///                             edge!(node_id!("left") => node_id!("right")).into()
///                         ],
///                    });
///         assert_eq!(subgraph!(; edge!(node_id!("left") => node_id!("right"))),
///                    Subgraph {
///                        id: Id::Anonymous("".to_string()),
///                        stmts: vec![
///                             edge!(node_id!("left") => node_id!("right")).into()
///                         ],
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

/// Constructs a node in dot lang.
///
/// # Arguments:
/// - prefix for id (optional): esc or html
/// - id(optional): the id for the node
/// - port(optional): the port of the node in a format `id => port`
/// - attributes(optional): either a vec of attributes or attributes in a form of variadic arguments
///
///
/// # Examples:
/// ```rust
///     fn node_test() {
///         use dot_generator::*;
///         use dot_structures::*;
///
///         assert_eq!(node!(), Node::new(NodeId(id!(), None), vec![]));
///
///         assert_eq!(node!(html "abc"; attr!("a","a")),
///                    Node::new(NodeId(id!(html "abc"), None),
///                              vec![attr!("a","a")]));
///
///         assert_eq!(node!(esc "abc"; attr!("a","a")),
///                    Node::new(NodeId(id!(esc "abc"), None),
///                              vec![attr!("a","a")]));
///
///         assert_eq!(node!("abc" ; attr!("a","a"), attr!("b","b")),
///                    Node::new(
///                         NodeId(id!(html "abc"), None),
///                         vec![attr!("a","a"),attr!("b","b")]
///                     )
///         );
///
///         assert_eq!(node!("abc" , vec![attr!("a","a"),attr!("a","a")]),
///                    Node::new(NodeId(id!( "abc"), None),
///                              vec![attr!("a","a"), attr!("a","a")])
///         );
///
///         assert_eq!(
///             node!("abc" => port!(,"id") ; attr!("a","a"),attr!("a","a")),
///             Node::new(
///                 NodeId(id!( "abc"), Some(Port(None,Some("id".to_string())))),
///                              vec![attr!("a","a"), attr!("a","a")])
///         );
///
///         assert_eq!(
///             node!(esc "abc" => port!(id!("port_id")) ;
///                     attr!("a","a"),attr!("a","a")),
///             Node::new(NodeId(id!(esc "abc"), Some(port!(id!("port_id")))),
///                              vec![attr!("a","a"), attr!("a","a")])
///         );
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

/// Constructs an edge.
///
/// # Arguments:
/// - chain of edges separated by `=>`: the edge expects either `node_id!` or `subgraph!`
/// - attributes: either a vec of attributes or attributes in a form of variadic arguments
///
/// # Examples:
///
/// ```rust
///     fn edge_test() {
///         use dot_generator::*;
///         use dot_structures::*;
///
///          assert_eq!(
///             edge!(node_id!("1") => node_id!("2")),
///             Edge {
///                 ty: EdgeTy::Pair(
///                         Vertex::N(node_id!("1")),
///                         Vertex::N(node_id!("2"))
///                     ),
///                 attributes: vec![]
///             }
///         );
///
///         assert_eq!(
///             edge!(
///                 node_id!("1") => node_id!("2") => subgraph!("a")
///             ),
///             Edge {
///                 ty: EdgeTy::Chain(vec![
///                         Vertex::N(node_id!("1")),
///                         Vertex::N(node_id!("2")),
///                         Vertex::S(subgraph!("a"))]),
///                 attributes: vec![] }
///         );
///
///         assert_eq!(
///             edge!(
///                 node_id!("1") => node_id!("2"), vec![attr!("a","b")]
///             ),
///             Edge {
///                 ty: EdgeTy::Pair(
///                         Vertex::N(node_id!("1")),
///                         Vertex::N(node_id!("2"))
///                     ),
///                 attributes: vec![attr!("a","b")] }
///         );
///
///         assert_eq!(
///             edge!(node_id!("1") => node_id!("2");
///                   attr!("a","b"), attr!("a1","b1")),
///             Edge {
///                 ty: EdgeTy::Pair(
///                         Vertex::N(node_id!("1")),
///                         Vertex::N(node_id!("2"))
///                     ),
///                 attributes: vec![attr!("a","b"), attr!("a1","b1")] }
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

/// Constructs a graph.
/// # Arguments
/// - prefix strict(optional): the prefix defines the strictness of the graph
/// - prefix di(optional): the prefix defines the graph is a digraph
/// - id: id of the graph. Can be constructed using `id!` macros
/// - a collection of statements or variadic arguments but the delegates from the statement
///
/// Therefore, the macros accepts two different forms where the first separator denotes it:
/// - `graph!(id, vec![statement])` where statement can be constructed using the `stmt!` macros
/// - `graph!(id; statement_delegate, statement_delegate2, ...)
///     where the delegates are undertaking structures. See `stmt! macros
///
/// # Note:
/// - the macros expects the statement delegates, e.g. nodes, edges, attributes. See `stmt!` macro
///
/// # Examples:
/// ```rust
///     fn graph_test() {
///         use dot_generator::*;
///         use dot_structures::*;
///
///          assert_eq!(
///             graph!(strict di id!("abc")),
///             Graph::DiGraph { id: id!("abc"), strict: true, stmts: vec![] }
///         );
///
///         assert_eq!(
///             graph!(strict di id!("abc");
///                     node!("abc"),
///                     node!("cde")
///             ),
///             Graph::DiGraph {
///                 id: id!("abc"),
///                 strict: true,
///                 stmts: vec![stmt!(node!("abc")),stmt!(node!("cde"))] }
///         );
///
///         assert_eq!(
///             graph!(di id!("abc");
///                     edge!(node_id!("a") => node_id!("b"); attr!("a","b")),
///                     subgraph!(
///                         "abc",
///                         vec![stmt!(node!("node1")), stmt!(attr!("a","b"))]
///                     )
///             ),
///             Graph::DiGraph {
///                 id: id!("abc"),
///                 strict: false,
///                 stmts: vec![
///                         edge!(node_id!("a") => node_id!("b"); attr!("a","b")).into(),
///                         subgraph!(
///                             "abc",
///                             vec![stmt!(node!("node1")), stmt!(attr!("a","b"))]
///                         ).into()
///                         ]
///             });
///
///         assert_eq!(
///             graph!(di id!("abc"),
///                     vec![
///                         stmt!(edge!(node_id!("a") => node_id!("b"); attr!("a","b"))),
///                         stmt!(node!("e")),
///                         stmt!(attr!("attr","val")),
///                     ]
///             ),
///             Graph::DiGraph {
///                 id: id!("abc"),
///                 strict: false,
///                 stmts: vec![
///                     edge!(node_id!("a") => node_id!("b"); attr!("a","b")).into(),
///                     node!("e").into(),
///                     attr!("attr","val").into()
///                 ] }
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
    fn port_test() {
        assert_eq!(
            port!(),
            Port(None,None)
        );
        assert_eq!(
            port!(,""),
            Port(None,Some("".to_string()))
        );
        assert_eq!(
            port!(id!(),""),
            Port(Some( Id::Anonymous("".to_string())),Some("".to_string()))
        );
        assert_eq!(
            port!(id!()),
            Port(Some( Id::Anonymous("".to_string())),None)
        );


    }

    #[test]
    fn node_id_test() {
        assert_eq!(
            node_id!(),
            NodeId(Id::Anonymous("".to_owned()), None)
        );

        assert_eq!(
            node_id!("plain"),
            NodeId(Id::Plain("plain".to_owned()), None)
        );

        assert_eq!(
            node_id!("plain", port!()),
            NodeId(Id::Plain("plain".to_owned()), Some(Port(None,None)))
        );

        assert_eq!(
            node_id!(esc "escaped"),
            NodeId(Id::Escaped("\"escaped\"".to_owned()),None)
        );
        assert_eq!(
            node_id!(html "</br>escaped"),
            NodeId(Id::Html("</br>escaped".to_owned()),None)
        );
    }

    #[test]
    fn graph_test() {
        assert_eq!(
            graph!(strict di id!("abc")),
            Graph::DiGraph {
                id: id!("abc"),
                strict: true,
                stmts: vec![]
            }
        );
        assert_eq!(
            graph!(strict di id!("abc");stmt!(node!("abc"))),
            Graph::DiGraph {
                id: id!("abc"),
                strict: true,
                stmts: vec![stmt!(node!("abc"))]
            }
        );
    }

    #[test]
    fn edge_test() {
        assert_eq!(
            edge!(node_id!("1") => node_id!("2")),
            Edge {
                ty: EdgeTy::Pair(Vertex::N(node_id!("1")), Vertex::N(node_id!("2"))),
                attributes: vec![]
            }
        );
        assert_eq!(
            edge!(node_id!("1") => node_id!("2") => subgraph!("a")),
            Edge {
                ty: EdgeTy::Chain(vec![
                    Vertex::N(node_id!("1")),
                    Vertex::N(node_id!("2")),
                    Vertex::S(subgraph!("a"))
                ]),
                attributes: vec![]
            }
        );
        assert_eq!(
            edge!(node_id!("1") => node_id!("2"), vec![attr!("a","b")]),
            Edge {
                ty: EdgeTy::Pair(Vertex::N(node_id!("1")), Vertex::N(node_id!("2"))),
                attributes: vec![attr!("a", "b")]
            }
        );
        assert_eq!(
            edge!(node_id!("1") => node_id!("2"); attr!("a","b")),
            Edge {
                ty: EdgeTy::Pair(Vertex::N(node_id!("1")), Vertex::N(node_id!("2"))),
                attributes: vec![attr!("a", "b")]
            }
        );
    }

    #[test]
    fn stmt_test() {
        assert_eq!(
            stmt!(node!()),
            Stmt::Node(Node::new(NodeId(id!(), None), vec![]))
        );
    }

    #[test]
    fn subgraph_test() {
        assert_eq!(
            subgraph!(),
            Subgraph {
                id: Id::Anonymous("".to_string()),
                stmts: vec![]
            }
        );
        assert_eq!(
            subgraph!("abc";node!()),
            Subgraph {
                id: Id::Plain("abc".to_string()),
                stmts: vec![stmt!(node!())],
            }
        );
    }

    #[test]
    fn node_test() {
        assert_eq!(node!(), Node::new(NodeId(id!(), None), vec![]));
        assert_eq!(
            node!(html "abc"; attr!("a","a")),
            Node::new(NodeId(id!(html "abc"), None), vec![attr!("a", "a")])
        );
        assert_eq!(
            node!(html "abc" ; attr!("a","a")),
            Node::new(NodeId(id!(html "abc"), None), vec![attr!("a", "a")])
        );
        assert_eq!(
            node!("abc" ; attr!("a","a"),attr!("a","a")),
            Node::new(
                NodeId(id!("abc"), None),
                vec![attr!("a", "a"), attr!("a", "a")]
            )
        )
    }

    #[test]
    fn attr_test() {
        assert_eq!(attr!("a", "1"), Attribute(id!("a"), id!("1")));
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
