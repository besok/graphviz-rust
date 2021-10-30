use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Port(pub Option<Id>, pub Option<String>);

#[derive(Debug, Clone, Eq)]
pub enum Id {
    Html(String),
    Escaped(String),
    Plain(String),
    Anonymous(String),
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Id::Html(v) => f.write_str(format!("html {}", v).as_str()),
            Id::Escaped(v) => f.write_str(format!("esc {}", v).as_str()),
            Id::Plain(v) => f.write_str(format!("{}", v).as_str()),
            Id::Anonymous(v) => f.write_str(format!("anon {}", v).as_str()),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct NodeId(pub Id, pub Option<Port>);

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
            _ => false
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Attribute(pub Id, pub Id);

#[derive(PartialEq, Debug, Clone)]
pub enum GraphAttributes {
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
pub struct Edge {
    pub ty: EdgeTy,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum EdgeTy {
    Pair(Vertex, Vertex),
    Chain(Vec<Vertex>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    pub id: NodeId,
    pub attributes: Vec<Attribute>,
}

impl Node {
    pub fn new(id: NodeId, attributes: Vec<Attribute>) -> Self {
        Node { id, attributes }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Stmt {
    Node(Node),
    Subgraph(Subgraph),
    Attribute(Attribute),
    GAttribute(GraphAttributes),
    Edge(Edge),
}

impl From<Node> for Stmt {
    fn from(v: Node) -> Self {
        Stmt::Node(v)
    }
}

impl From<Edge> for Stmt {
    fn from(v: Edge) -> Self {
        Stmt::Edge(v)
    }
}

impl From<GraphAttributes> for Stmt {
    fn from(v: GraphAttributes) -> Self {
        Stmt::GAttribute(v)
    }
}

impl From<Attribute> for Stmt {
    fn from(v: Attribute) -> Self {
        Stmt::Attribute(v)
    }
}

impl From<Subgraph> for Stmt {
    fn from(v: Subgraph) -> Self {
        Stmt::Subgraph(v)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Subgraph {
    pub id: Id,
    pub stmts: Vec<Stmt>,
}


#[derive(PartialEq, Debug, Clone)]
pub enum Vertex {
    N(NodeId),
    S(Subgraph),
}

impl From<NodeId> for Vertex {
    fn from(v: NodeId) -> Self {
        Vertex::N(v)
    }
}

impl From<Subgraph> for Vertex {
    fn from(v: Subgraph) -> Self {
        Vertex::S(v)
    }
}

#[derive(Debug, PartialEq)]
pub enum Graph {
    Graph { id: Id, strict: bool, stmts: Vec<Stmt> },
    DiGraph { id: Id, strict: bool, stmts: Vec<Stmt> },
}
