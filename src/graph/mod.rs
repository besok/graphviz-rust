enum GraphType {
    Graph,
    DiGraph,
}

struct Graph<'a> {
    id: Id,
    ty: GraphType,
    strict: bool,
    vertices: Vec<Vertex>,
    edges: Vec<Edge<'a>>,
    attributes:Vec<dyn Attribute>
}

enum Id {
    Html(String),
    Escaped(String),
    Plain(Stringgrammar)
}

enum Vertex {
    Node {
        id: Id,
        attributes:Vec<dyn Attribute>,
    },
    Subgraph {
        id: Id,
        attributes:Vec<dyn Attribute>
    },
}

struct Edge<'a> {
    from: &'a Vertex,
    to: &'a Vertex,
    attributes:Vec<dyn Attribute>,
}

enum AttributeOwner {
    Edges,
    Nodes,
    Root,
    Subgraph,
    Cluster,
}

trait Attribute {
    fn owners(&self) -> Vec<AttributeOwner>;
}