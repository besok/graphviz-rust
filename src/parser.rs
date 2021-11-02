//! Parser for the ['notation'].
//!
//! The grammar can be viewed in `/grammar/dot.pest`
//!
//! ['notation']: https://graphviz.org/doc/info/lang.html
use std::collections::HashMap;
use std::iter::Map;
use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::RuleType;
use crate::pest::Parser;
use dot_structures::*;

#[derive(Parser)]
#[grammar = "grammar/dot.pest"]
struct DotParser;


pub(crate) fn parse(dot: &str) -> Result<Graph, String> {
    do_parse(dot, Rule::file)
        .map(|r| r.into_iter().next().unwrap())
        .map(|r| process_graph(down(r)))
        .map_err(|v| v.to_string())
}

fn down(rule: Pair<Rule>) -> Pair<Rule> {
    rule.into_inner().next().unwrap()
}

fn do_parse(input: &str, ty: Rule) -> Result<Pairs<Rule>, Error<Rule>> { DotParser::parse(ty, input) }

fn process_attr_list(rule: Pair<Rule>) -> Vec<Attribute> {
    let mut attrs = vec![];
    let mut attr_list = rule.into_inner();
    while attr_list.peek().is_some() {
        attrs.push(process_attr(attr_list.next().unwrap()))
    }
    attrs
}

fn process_bare_attr(rule: Pair<Rule>) -> Attribute {
    let mut attr = rule.into_inner();
    let key = attr.next().map(process_id).unwrap();
    let val = attr.next().map(process_id).unwrap();
    Attribute(key, val)
}

fn process_attr(rule: Pair<Rule>) -> Attribute {
    process_bare_attr(down(rule))
}

fn process_id(rule: Pair<Rule>) -> Id {
    let val = rule.as_str().to_string();
    match down(rule).as_rule() {
        Rule::plain => Id::Plain(val),
        Rule::html => Id::Html(val),
        Rule::string_qt => Id::Escaped(val),
        p => panic!("unreachable, got {:?}", p)
    }
}

fn process_port(port: Pair<Rule>) -> Port {
    let mut port_r = port.into_inner();
    if let Some(r) = port_r.next() {
        let mut id = None;
        let mut com = None;

        match r.as_rule() {
            Rule::compass => com = Some(r.as_str().to_string()),
            Rule::id => {
                id = Some(process_id(r)).map(|id| id);
                if let Some(r) = port_r.next() {
                    com = Some(r.as_str().to_string());
                }
            }
            _ => panic!("unreachable!")
        }
        Port(id, com)
    } else {
        panic!("port can not be empty")
    }
}

fn process_node_id(rule: Pair<Rule>) -> NodeId {
    let mut node_id = rule.into_inner();
    let id = node_id.next().map(process_id).unwrap();
    if let Some(r) = node_id.next() {
        NodeId(id, Some(process_port(r)))
    } else {
        NodeId(id, None)
    }
}

fn process_subgraph(rule: Pair<Rule>) -> Subgraph {
    let mut sub_r = rule.into_inner();
    let id = match sub_r.peek().map(|r| r.as_rule()) {
        Some(Rule::id) => process_id(sub_r.next().unwrap()),
        _ => Id::Anonymous(rand::random::<usize>().to_string())
    };
    let stmts = process_body(sub_r.next().unwrap());
    Subgraph { id, stmts }
}

fn process_body(rule: Pair<Rule>) -> Vec<Stmt> {
    let mut stmts = vec![];
    let mut body_r = rule.into_inner();
    while body_r.peek().is_some() {
        stmts.push(process_stmt(body_r.next().unwrap()));
    }
    stmts
}

fn process_node(rule: Pair<Rule>) -> Node {
    let mut node_r = rule.into_inner();
    let id = process_node_id(node_r.next().unwrap());
    if let Some(r) = node_r.next() {
        Node { id, attributes: process_attr_list(r) }
    } else {
        Node { id, attributes: vec![] }
    }
}

fn process_vertex(rule: Pair<Rule>) -> Vertex {
    let vertex_r = down(rule);
    match vertex_r.as_rule() {
        Rule::node_id => Vertex::N(process_node_id(vertex_r)),
        Rule::subgraph => Vertex::S(process_subgraph(vertex_r)),
        _ => unreachable!("")
    }
}

fn process_edge<'a>(rule: Pair<Rule>) -> Vec<Vertex> {
    let mut edge_r = rule.into_inner();
    let h = process_vertex(edge_r.next().unwrap());
    let mut chain = vec![h];

    while edge_r.peek().is_some() {
        chain.push(process_vertex(down(edge_r.next().unwrap())))
    }
    chain
}

fn process_edge_stmt(rule: Pair<Rule>) -> Edge {
    let mut edge_r = rule.into_inner();
    let edges = process_edge(edge_r.next().unwrap());

    let ty = if edges.len() > 2 {
        EdgeTy::Chain(edges)
    } else {
        let mut edge_iter = edges.into_iter();
        EdgeTy::Pair(edge_iter.next().unwrap(), edge_iter.next().unwrap())
    };

    if let Some(attr_r) = edge_r.next() {
        Edge { ty, attributes: process_attr_list(attr_r) }
    } else {
        Edge { ty, attributes: vec![] }
    }
}

fn process_attr_stmt(rule: Pair<Rule>) -> GraphAttributes {
    let mut stmts_r = rule.into_inner();
    let mark = stmts_r.next().unwrap().as_str();
    let attrs = process_attr_list(stmts_r.next().unwrap());
    GraphAttributes::new(mark, attrs)
}

fn process_stmt(rule: Pair<Rule>) -> Stmt {
    let stmt_r = down(rule);
    match stmt_r.as_rule() {
        Rule::attr_stmt => Stmt::GAttribute(process_attr_stmt(stmt_r)),
        Rule::subgraph => Stmt::Subgraph(process_subgraph(stmt_r)),
        Rule::node => Stmt::Node(process_node(stmt_r)),
        Rule::bare_attr => Stmt::Attribute(process_bare_attr(stmt_r)),
        Rule::edge_stmt => Stmt::Edge(process_edge_stmt(stmt_r)),
        _ => unreachable!()
    }
}

fn process_graph(rule: Pair<Rule>) -> Graph {
    let mut graph_r = rule.into_inner();
    let strict = match graph_r.peek().map(|r| r.as_rule()) {
        Some(Rule::strict) => {
            graph_r.next();
            true
        }
        _ => false
    };

    let is_di = match graph_r.next().map(|r| r.as_str()) {
        Some("digraph") => true,
        _ => false
    };

    let id = match graph_r.peek().map(|r| {
        r.as_rule()
    }) {
        Some(Rule::id) => process_id(graph_r.next().unwrap()),
        _ => Id::Anonymous(rand::random::<usize>().to_string())
    };

    let stmts = process_body(graph_r.next().unwrap());
    if is_di {
        Graph::DiGraph { id, strict, stmts }
    } else {
        Graph::Graph { id, strict, stmts }
    }
}

#[cfg(test)]
mod test {
    use pest::error::Error;
    use pest::iterators::{Pair, Pairs};
    use pest::RuleType;


    use crate::parser::{do_parse, DotParser, down, parse, process_attr, process_attr_list, process_attr_stmt, process_edge, process_edge_stmt, process_id, process_node, process_node_id, process_stmt, process_vertex, Stmt, Vertex};
    use crate::parser::Rule;
    use crate::pest::Parser;

    use dot_generator::{id, port, attr, node, stmt, subgraph, graph, edge, node_id};
    use dot_structures::*;

    fn _parse(input: &str, ty: Rule) -> Pair<Rule> {
        match do_parse(input, ty) {
            Ok(mut r) => r.next().unwrap(),
            Err(e) => panic!("parsing err: {}", e)
        }
    }


    #[test]
    fn id_test() {
        let result = process_id(_parse("abc_a", Rule::id));
        assert_eq!(result, id!("abc_a"));

        let result = process_id(_parse("\"ab\\\"c\"", Rule::id));
        assert_eq!(result, id!(esc "\"ab\\\"c\""));

        let result = process_id(_parse(r#"<<IMG SCALE="FAL" SRC="value" /></B>abc </B>>"#, Rule::id));
        assert_eq!(result, id!(html r#"<<IMG SCALE="FAL" SRC="value" /></B>abc </B>>"#));
    }

    #[test]
    fn attr_test() {
        let result = process_attr(_parse("a=1", Rule::attr));
        assert_eq!(result, attr!("a","1"));
        let result = process_attr(_parse("a = 1 , ;", Rule::attr));
        assert_eq!(result, attr!("a","1"));
    }

    #[test]
    fn attr_list_test() {
        let result = process_attr_list(_parse("[a=1 , b=c ; d=<<abc>> e=e]", Rule::attr_list));
        let expect = vec![
            attr!("a", "1"),
            attr!("b", "c"),
            attr!("d", html "<<abc>>"),
            attr!("e", "e"),
        ];
        assert_eq!(result, expect);
        let result = process_attr_list(_parse("[a=1 , b=c] [ d=<<abc>> e=e]", Rule::attr_list));
        assert_eq!(result, expect);
    }

    #[test]
    fn node_id_test() {
        let result = process_node_id(_parse("abc:n", Rule::node_id));
        let expect = node_id!(id!("abc"), port!(, "n"));
        assert_eq!(result, expect);

        let result = process_node_id(_parse("abc:abc", Rule::node_id));
        let expect = node_id!(id!("abc"), port!(  id!("abc")));
        assert_eq!(result, expect);

        let result = process_node_id(_parse("abc:abc:n", Rule::node_id));
        let expect = node_id!(id!("abc"), port!(id!("abc"),"n"));
        assert_eq!(result, expect);
    }

    #[test]
    fn node_test() {
        let result = process_node(_parse("abc:n[a=1 , b=c ; d=<<abc>> e=e]", Rule::node));
        let p = port!(, "n" );
        let attributes = vec![
            attr!("a", "1"),
            attr!("b", "c"),
            attr!("d", html "<<abc>>"),
            attr!("e", "e"),
        ];
        assert_eq!(result, node!("abc" => p, attributes));
    }

    #[test]
    fn attr_stmts_test() {
        let result = process_attr_stmt(_parse("node [a=1 , b=c ; d=<<abc>> e=e]", Rule::attr_stmt));
        let attributes = vec![
            attr!("a", "1"),
            attr!("b", "c"),
            attr!("d", html "<<abc>>"),
            attr!("e", "e"),
        ];
        assert_eq!(result, GraphAttributes::Node(attributes));

        let result = process_attr_stmt(_parse("graph [a=1 , b=c ; d=<<abc>> e=e]", Rule::attr_stmt));
        let attributes = vec![
            attr!("a", "1"),
            attr!("b", "c"),
            attr!("d", html "<<abc>>"),
            attr!("e", "e"),
        ];
        assert_eq!(result, GraphAttributes::Graph(attributes));
    }

    #[test]
    fn vertex_test() {
        let result = process_vertex(_parse("node", Rule::vertex));
        assert_eq!(result, Vertex::N(node_id!("node")));
    }

    #[test]
    fn edge_test() {
        let result = process_edge(_parse("node -> node1 -> node2", Rule::edge));
        let expected = vec![
            Vertex::N(node_id!("node")),
            Vertex::N(node_id!("node1")),
            Vertex::N(node_id!("node2")),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn edge_stmt_test() {
        let result = process_edge_stmt(_parse("node -> node1 -> node2[a=2]", Rule::edge_stmt));
        assert_eq!(result, edge!(node_id!("node")=> node_id!("node1")=>node_id!("node2"); attr!("a","2")));

        let result = process_edge_stmt(_parse("node -> subgraph sg{a -> b}[a=2]", Rule::edge_stmt));

        assert_eq!(result, edge!(
            node_id!("node") => subgraph!("sg";stmt!(edge!(node_id!("a") => node_id!("b"))));
            attr!("a","2")
        ));
    }

    #[test]
    fn stmt_test() {
        let result = process_stmt(_parse("a=b", Rule::stmt));
        assert_eq!(result, stmt!(attr!("a","b")));

        let result = process_stmt(_parse("node [a=1 , b=c ; d=<<abc>> e=e]", Rule::stmt));
        let attributes = vec![
            attr!("a", "1"),
            attr!("b", "c"),
            attr!("d", html "<<abc>>"),
            attr!("e", "e"),
        ];
        assert_eq!(result, stmt!(GraphAttributes::Node(attributes)));

        let result = process_stmt(_parse("node -> node1 -> node2[a=2]", Rule::stmt));

        assert_eq!(result, stmt!( edge!(node_id!("node")=> node_id!("node1")=>node_id!("node2"); attr!("a","2"))));
    }

    #[test]
    fn graph_test() {
        let g: Graph = parse(r#"
        strict digraph t {
            aa[color=green]
            subgraph v {
	            aa[shape=square]
	            subgraph vv{a2 -> b2}
	            aaa[color=red]
	            aaa -> bbb
            }
            aa -> be -> subgraph v { d -> aaa}
            aa -> aaa -> v
        }
        "#).unwrap();

        assert_eq!(
            g,
            graph!(strict di id!("t");
              node!("aa";attr!("color","green")),
              subgraph!("v";
                node!("aa"; attr!("shape","square")),
                subgraph!("vv"; edge!(node_id!("a2") => node_id!("b2"))),
                node!("aaa";attr!("color","red")),
                edge!(node_id!("aaa") => node_id!("bbb"))
                ),
              edge!(node_id!("aa") => node_id!("be") => subgraph!("v"; edge!(node_id!("d") => node_id!("aaa")))),
              edge!(node_id!("aa") => node_id!("aaa") => node_id!("v"))
            )
        )
    }

    #[test]
    fn comments_test() {
        let g: Graph = parse("// abc \n # abc \n strict digraph t { \n /* \n abc */ \n}").unwrap();

        assert_eq!(g, graph!(strict di id!("t")))
    }
}