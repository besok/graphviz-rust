//! # The structures of dot language
//! The set of components of the graphviz dot notation
//! endeavouring to follow comparatively close to the language [`notation`]
//!
//! [`notation`]: https://graphviz.org/doc/info/lang.html
//!
//! # Description:
//! ```txt
//!     strict digraph t {           <= graph
//!         aa[color=green]          <= node aa and attributes in [..]
//!         subgraph v {             <= subgraph v
//! 	     aa[shape=square]
//! 	     subgraph vv{a2 -> b2}
//! 	     aaa[color=red]
//! 	     aaa -> subgraph { d -> aaa}  <= subgraph id is anon
//!         }
//!        aa -> be -> d -> aaa       <= type of the edge is chain
//!    }
//! ```
use std::fmt::{Display, Formatter};

/// the component represents a port in the language.
/// It contains from id and direction. All can be optional separately but not at the same time.
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Port(pub Option<Id>, pub Option<String>);

/// the component represents a id in the language.
/// The Anonymous is a virtual component to keep the other components consistent in case
/// when a node or subgraph is anonymous
#[derive(Debug, Clone,PartialEq,Eq)]
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

/// the component represents a node_id in the language.
/// The component turns up in the edges predominantly or as an id for a node.
#[derive(Debug, PartialEq, Clone)]
pub struct NodeId(pub Id, pub Option<Port>);

/// the component represents a attribute in the language.
#[derive(PartialEq, Debug, Clone)]
pub struct Attribute(pub Id, pub Id);

/// the component represents a set of attributes with prefix denoting a type in the language.
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

/// the component represents a edge in the language.
#[derive(Debug, PartialEq, Clone)]
pub struct Edge {
    pub ty: EdgeTy,
    pub attributes: Vec<Attribute>,
}

impl Edge {
    fn add_attr(&mut self,attr:Attribute){
        self.attributes.push(attr)
    }
}

/// the component depicts a type of the edge, namely it is a pair of chain.
/// From the graph point of view, it impacts a compact display only.
#[derive(Debug, PartialEq, Clone)]
pub enum EdgeTy {
    Pair(Vertex, Vertex),
    Chain(Vec<Vertex>),
}
/// the component represents the vital component, namely node in the lang.
#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    pub id: NodeId,
    pub attributes: Vec<Attribute>,
}

impl Node {
    pub fn new(id: NodeId, attributes: Vec<Attribute>) -> Self {
        Node { id, attributes }
    }
    fn add_attr(&mut self,attr:Attribute){
        self.attributes.push(attr)
    }
}

/// the component represents a wrapper to keep sustainability in subgraph and graph bodies.
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
/// the component represents a subgraph  in the lang.
#[derive(PartialEq, Debug, Clone)]
pub struct Subgraph {
    pub id: Id,
    pub stmts: Vec<Stmt>,
}

impl Subgraph {
    fn add_stmt(&mut self,stmt:Stmt){
        self.stmts.push(stmt)
    }
}

/// the component represents an edge component.
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
/// the component represents a graph in the lang.
#[derive(Debug, PartialEq,Clone)]
pub enum Graph {
    Graph { id: Id, strict: bool, stmts: Vec<Stmt> },
    DiGraph { id: Id, strict: bool, stmts: Vec<Stmt> },
}

impl Graph {
    pub fn add_stmt(&mut self,stmt:Stmt){
        match self {
            Graph::Graph { stmts,.. } => {stmts.push(stmt)}
            Graph::DiGraph { stmts,.. } => {stmts.push(stmt)}
        }
    }
}
