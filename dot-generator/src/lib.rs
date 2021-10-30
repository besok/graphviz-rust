use dot_structures::*;

#[macro_export]
macro_rules! port {
    () => {Port(None,None)};
    ( , $str:expr) => { Port(None,Some($str.to_string()))};
    ( $id:expr , $str:expr) => {Port(Some($id),Some($str.to_string()))};
    ( $id:expr) => {Port(Some($id),None)};
}

#[macro_export]
macro_rules! node_id {
    () => {  NodeId(id!(),None) };
    ($e:expr) => { NodeId(id!($e),None) };
    ($e:expr, $p:expr) => { NodeId(id!($e),Some($p)) };
    ($i:ident $e:expr) => { NodeId(id!($i$e),None) };
    ($i:ident $e:expr, $p:expr) => { NodeId(id!($i$e),Some($p)) };
}

#[macro_export]
macro_rules! id {
    () => { Id::Anonymous("".to_string()) };
    (html$e:expr) => { Id::Html(format!("{}",$e))};
    (esc$e:expr) => { Id::Escaped(format!("{}",$e))};
    ($e:expr) => { Id::Plain(format!("{}",$e))};
}

#[macro_export]
macro_rules! attr {
    ($ik:ident $k:expr,$iv:ident $v:expr) => {Attribute(id!($k),id!($iv $v))};
    ($ik:ident $k:expr,$v:expr) => {Attribute(id!($ik $k),id!($v))};
    ($k:expr, $iv:ident $v:expr) => {Attribute(id!($k),id!($iv $v))};
    ($k:expr,$v:expr) => {Attribute(id!($k),id!($v))}
}
#[macro_export]
macro_rules! stmt {
    ($k:expr) => {Stmt::from($k)};
}

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

}


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
        assert_eq!(id!(esc "\"ab\\\"c\""), Id::Escaped("\"ab\\\"c\"".to_string()));
    }
}
