use std::hash::{Hash, Hasher};

mod parser;
mod macros;

#[derive(Debug, PartialEq, Clone, Eq)]
 struct Port(pub Option<Box<Id>>, pub Option<String>);

#[derive(Debug, Clone, Eq)]
 enum Id {
    Html(String),
    Escaped(String),
    Plain(String),
    IdwPort(Box<Id>, Option<Port>),
    Anonymous(String),
}

impl Hash for Id {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Id::Html(s) => {
                state.write("html".as_bytes());
                state.write(s.as_bytes());
            }
            Id::Escaped(s) => {
                state.write("escaped".as_bytes());
                state.write(s.as_bytes());
            }
            Id::Plain(s) => {
                state.write("plain".as_bytes());
                state.write(s.as_bytes());
            }
            Id::IdwPort(id, Some(Port(id_opt, str_opt))) => {
                state.write("withPort".as_bytes());
                id.hash(state);
                if let Some(id_p) = id_opt {
                    id_p.hash(state)
                }
                if let Some(str) = str_opt {
                    state.write(str.as_bytes())
                }
            }
            Id::IdwPort(id, None) => {
                state.write("withPort".as_bytes());
                id.hash(state);
            }
            Id::Anonymous(s) => {
                state.write("anon".as_bytes());
                state.write(s.as_bytes());
            }
        }
    }
}

impl PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Id::Html(l), Id::Html(r))
            | (Id::Escaped(l), Id::Escaped(r))
            | (Id::Plain(l), Id::Plain(r))
            | (Id::Anonymous(l), Id::Anonymous(r)) => l == r,
            (Id::IdwPort(idl, pl), Id::IdwPort(idr, pr)) => idl == idr && pl == pr,
            (Id::IdwPort(idl, None), id)
            | (id, Id::IdwPort(idl, None)) => (&**idl) == id,
            _ => false
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
 enum Attribute {
    Arbitrary(Id, Id)
}

#[derive(PartialEq, Debug, Clone)]
 enum GraphAttributes {
    Graph(Vec<Attribute>),
    Node(Vec<Attribute>),
    Edge(Vec<Attribute>),
}

impl GraphAttributes {
    pub fn new(ty: &str, attrs: Vec<Attribute>) -> Self {
        match ty.to_lowercase().as_str() {
            "graph" => GraphAttributes::Graph(attrs),
            "node" => GraphAttributes::Node(attrs),
            "edge" => GraphAttributes::Edge(attrs),
            _ => panic!("only graph, node, edge is applied here. ")
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
 struct Edge {
    ty: EdgeTy,
    attributes: Vec<Attribute>,
}

#[derive(Debug, PartialEq, Clone)]
enum EdgeTy {
    Pair(Vertex, Vertex),
    Chain(Vec<Vertex>),
}



 enum AttributeOwner {
    Edges,
    Nodes,
    Root,
    Subgraph,
    Cluster,
}

#[derive(Debug, PartialEq, Clone)]
 struct Node {
    id: Id,
    attributes: Vec<Attribute>,
}

impl Node {
    pub fn new(id: Id, attributes: Vec<Attribute>) -> Self {
        Node { id, attributes }
    }
}

#[derive(PartialEq, Debug,Clone)]
enum Stmt {
    Node(Node),
    Subgraph(Subgraph),
    Attribute(Attribute),
    GAttribute(GraphAttributes),
    Edge(Edge),
}
#[derive(PartialEq, Debug,Clone)]
struct Subgraph {
    id: Id,
    stmts: Vec<Stmt>,
}


#[derive(PartialEq, Debug,Clone)]
enum Vertex {
    N(Node),
    S(Subgraph),
}

#[derive(Debug, PartialEq)]
 pub enum Graph{
    Graph { id: Id, strict: bool, stmts: Vec<Stmt> },
    DiGraph { id: Id, strict: bool, stmts: Vec<Stmt> },
}
