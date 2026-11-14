//! Parser for the ['notation'].
//!
//! The grammar can be viewed in `/grammar/dot.pest`
//!
//! ['notation']: https://graphviz.org/doc/info/lang.html
use dot_structures::*;
use pest::{
    error::Error,
    iterators::{Pair, Pairs},
};

use crate::pest::Parser;

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

fn do_parse(input: &str, ty: Rule) -> Result<Pairs<'_, Rule>, Error<Rule>> {
    DotParser::parse(ty, input)
}

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
    let inner = down(rule);
    match inner.as_rule() {
        Rule::plain => Id::Plain(val),
        Rule::html => Id::Html(val),
        // DOT concatenates quoted contents, retaining attribute escapes.
        Rule::quoted_id => {
            let mut joined = String::from("\"");
            for part in inner.into_inner() {
                let raw = part.as_str();
                let mut chars = raw[1..raw.len() - 1].chars().peekable();
                while let Some(c) = chars.next() {
                    if c == '\\' {
                        match chars.peek() {
                            Some('\n') => {
                                chars.next();
                                continue;
                            }
                            Some('\r') => {
                                chars.next();
                                if chars.peek() == Some(&'\n') {
                                    chars.next();
                                }
                                continue;
                            }
                            Some(_) => {
                                joined.push(c);
                                joined.push(chars.next().unwrap());
                                continue;
                            }
                            None => {}
                        }
                    }
                    joined.push(c);
                }
            }
            joined.push('"');
            Id::Escaped(joined)
        }
        p => panic!("unreachable, got {:?}", p),
    }
}

fn parse_compass_manually(id: Id) -> Option<String> {
    match id {
        Id::Plain(ref s) => match s.as_str() {
            "n" | "ne" | "e" | "se" | "s" | "sw" | "w" | "nw" | "c" | "_" => Some(id.to_string()),
            _ => None
        }
        _ => None
    }
}

fn process_port(port: Pair<Rule>) -> Port {
    let mut port_r = port.into_inner();
    if let Some(r) = port_r.next() {
        match r.as_rule() {
            Rule::compass => Port(None, Some(r.as_str().to_string())),
            Rule::id => {
                let mb_id_mb_compass = process_id(r);
                if let Some(r) = port_r.next() {
                    let compass = match process_id(down(r)) {
                        Id::Plain(v) | Id::Escaped(v) | Id::Html(v) | Id::Anonymous(v) => v,
                    };
                    Port(Some(mb_id_mb_compass), Some(compass))
                } else {
                    parse_compass_manually(mb_id_mb_compass.clone())
                        .map(|s| Port(None, Some(s)))
                        .unwrap_or_else(|| Port(Some(mb_id_mb_compass), None))
                }
            }
            _ => panic!("unreachable!"),
        }
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
    if matches!(sub_r.peek().map(|r| r.as_rule()), Some(Rule::subgraph_keyword)) {
        sub_r.next();
    }
    let id = match sub_r.peek().map(|r| r.as_rule()) {
        Some(Rule::id) => process_id(sub_r.next().unwrap()),
        _ => Id::Anonymous(String::new()),
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
        Node {
            id,
            attributes: process_attr_list(r),
        }
    } else {
        Node {
            id,
            attributes: vec![],
        }
    }
}

fn process_vertex(rule: Pair<Rule>) -> Vertex {
    let vertex_r = down(rule);
    match vertex_r.as_rule() {
        Rule::node_id => Vertex::N(process_node_id(vertex_r)),
        Rule::subgraph => Vertex::S(process_subgraph(vertex_r)),
        _ => unreachable!(""),
    }
}

fn process_edge(rule: Pair<Rule>) -> Vec<Vertex> {
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
        Edge {
            ty,
            attributes: process_attr_list(attr_r),
        }
    } else {
        Edge {
            ty,
            attributes: vec![],
        }
    }
}

fn process_attr_stmt(rule: Pair<Rule>) -> GraphAttributes {
    let mut stmts_r = rule.into_inner();
    let mark = stmts_r.next().unwrap().as_str().to_ascii_lowercase();
    let attrs = process_attr_list(stmts_r.next().unwrap());
    GraphAttributes::new(&mark, attrs)
}

fn process_stmt(rule: Pair<Rule>) -> Stmt {
    let stmt_r = down(rule);
    match stmt_r.as_rule() {
        Rule::attr_stmt => Stmt::GAttribute(process_attr_stmt(stmt_r)),
        Rule::subgraph => Stmt::Subgraph(process_subgraph(stmt_r)),
        Rule::node => Stmt::Node(process_node(stmt_r)),
        Rule::bare_attr => Stmt::Attribute(process_bare_attr(stmt_r)),
        Rule::edge_stmt => Stmt::Edge(process_edge_stmt(stmt_r)),
        _ => unreachable!(),
    }
}

fn process_graph(rule: Pair<Rule>) -> Graph {
    let mut graph_r = rule.into_inner();
    let strict = match graph_r.peek().map(|r| r.as_rule()) {
        Some(Rule::strict) => {
            graph_r.next();
            true
        }
        _ => false,
    };

    let is_di = graph_r.next().unwrap().as_str().eq_ignore_ascii_case("digraph");

    let id = match graph_r.peek().map(|r| r.as_rule()) {
        Some(Rule::id) => process_id(graph_r.next().unwrap()),
        _ => Id::Anonymous(String::new()),
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
    #[test]
    fn parsing_the_same_source_twice_is_stable() {
        for src in ["digraph { }", "digraph { subgraph { a } }",
                    "digraph G { subgraph { a } }", "digraph G { subgraph S { a } }"] {
            assert_eq!(parse(src).unwrap(), parse(src).unwrap(), "L1 violated by: {src}");
        }
    }

    #[test]
    fn indentation_does_not_change_the_graph() {
        let a = parse("digraph { subgraph { a } }").unwrap();
        let b = parse("digraph {\n    subgraph {\n        a\n    }\n}").unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn parsed_and_constructed_agree() {
        let parsed = parse("digraph { subgraph { a } }").unwrap();
        let built = Graph::DiGraph {
            id: id!(),
            strict: false,
            stmts: vec![Stmt::Subgraph(Subgraph {
                id: id!(),
                stmts: vec![Stmt::Node(node!("a"))],
            })],
        };
        assert_eq!(parsed, built);
    }

    use dot_generator::{attr, edge, graph, id, node, node_id, port, stmt, subgraph};
    use dot_structures::*;
    use pest::iterators::Pair;

    use crate::parser::{
        do_parse, parse, process_attr, process_attr_list, process_attr_stmt, process_edge,
        process_edge_stmt, process_id, process_node, process_node_id, process_stmt, process_vertex,
        Rule, Stmt, Vertex,
    };

    fn _parse(input: &str, ty: Rule) -> Pair<Rule> {
        match do_parse(input, ty) {
            Ok(mut r) => r.next().unwrap(),
            Err(e) => panic!("parsing err: {}", e),
        }
    }

    #[test]
    fn id_test() {
        let result = process_id(_parse("abc_a", Rule::id));
        assert_eq!(result, id!("abc_a"));

        // valid ID
        let result = process_id(_parse(r#""a\b\c.'\"""#, Rule::id));
        assert_eq!(result, id!(esc r#"a\b\c.'\""#));

        // invalid ID unescaped quote
        let result = process_id(_parse(r#""a\b"\c.'\"""#, Rule::id));
        assert_eq!(result, id!(esc r#"a\b"#));

        let result = process_id(_parse("\"ab\\\"c\"", Rule::id));
        assert_eq!(result, id!(esc "ab\\\"c"));

        let result = process_id(_parse(
            r#"<<IMG SCALE="FAL" SRC="value" /></B>abc </B>>"#,
            Rule::id,
        ));
        assert_eq!(
            result,
            id!(html r#"<<IMG SCALE="FAL" SRC="value" /></B>abc </B>>"#)
        );

        let result = process_id(_parse(
            r#"<<TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
                          <TR><TD>left</TD><TD PORT="f1">mid dle</TD><TD PORT="f2">right</TD></TR>
                        </TABLE>>"#,
            Rule::id,
        ));
        assert_eq!(
            result,
            id!(html r#"<<TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
                          <TR><TD>left</TD><TD PORT="f1">mid dle</TD><TD PORT="f2">right</TD></TR>
                        </TABLE>>"#)
        );

        let result = process_id(_parse(
            r#"<
        <TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
                          <TR><TD>left</TD><TD PORT="f1">mid dle</TD><TD PORT="f2">right</TD></TR>
                        </TABLE>
                        >"#,
            Rule::id,
        ));
        assert_eq!(
            result,
            id!(html r#"<
        <TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
                          <TR><TD>left</TD><TD PORT="f1">mid dle</TD><TD PORT="f2">right</TD></TR>
                        </TABLE>
                        >"#)
        );
        let result = process_id(_parse(
            r#"<<tr><td>address_id:!@#$%^&*()_+/.,"\| int</td></tr>>"#,
            Rule::id,
        ));
        assert_eq!(
            result,
            id!(html r#"<<tr><td>address_id:!@#$%^&*()_+/.,"\| int</td></tr>>"#)
        );
    }

    #[test]
    fn attr_test() {
        let result = process_attr(_parse("a=1", Rule::attr));
        assert_eq!(result, attr!("a", "1"));
        let result = process_attr(_parse("a = 1 , ;", Rule::attr));
        assert_eq!(result, attr!("a", "1"));
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
        let expect = node_id!(id!("abc"), port!(id!("abc")));
        assert_eq!(result, expect);

        let result = process_node_id(_parse("abc:abc:n", Rule::node_id));
        let expect = node_id!(id!("abc"), port!(id!("abc"), "n"));
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

        let result =
            process_attr_stmt(_parse("graph [a=1 , b=c ; d=<<abc>> e=e]", Rule::attr_stmt));
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
        assert_eq!(
            result,
            edge!(node_id!("node")=> node_id!("node1")=>node_id!("node2"); attr!("a","2"))
        );

        let result = process_edge_stmt(_parse("node -> subgraph sg{a -> b}[a=2]", Rule::edge_stmt));

        assert_eq!(
            result,
            edge!(
                node_id!("node") => subgraph!("sg";stmt!(edge!(node_id!("a") => node_id!("b"))));
                attr!("a","2")
            )
        );
    }

    #[test]
    fn stmt_test() {
        let result = process_stmt(_parse("a=b", Rule::stmt));
        assert_eq!(result, stmt!(attr!("a", "b")));

        let result = process_stmt(_parse("node [a=1 , b=c ; d=<<abc>> e=e]", Rule::stmt));
        let attributes = vec![
            attr!("a", "1"),
            attr!("b", "c"),
            attr!("d", html "<<abc>>"),
            attr!("e", "e"),
        ];
        assert_eq!(result, stmt!(GraphAttributes::Node(attributes)));

        let result = process_stmt(_parse("node -> node1 -> node2[a=2]", Rule::stmt));

        assert_eq!(
            result,
            stmt!(edge!(node_id!("node")=> node_id!("node1")=>node_id!("node2"); attr!("a","2")))
        );
    }

    #[test]
    fn graph_html_test() {
        let g: Graph = parse(
            r#"
        digraph G {
        a [ label=< <TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
                    <TR><TD ROWSPAN="3" BGCOLOR="yellow">class</TD></TR>
                    <TR><TD PORT="here" BGCOLOR="lightblue">qualifier</TD></TR>
                    </TABLE>>
           ]
        b [shape=ellipse style=filled
            label=<
            <TABLE BGCOLOR="bisque">
            <TR>
                <TD COLSPAN="3">elephant</TD>
                <TD ROWSPAN="2" BGCOLOR="chartreuse"
                VALIGN="bottom" ALIGN="right">two</TD>
            </TR>
            <TR>
                <TD COLSPAN="2" ROWSPAN="2">
                    <TABLE BGCOLOR="grey">
                        <TR><TD>corn</TD></TR>
                        <TR><TD BGCOLOR="yellow">c</TD></TR>
                        <TR><TD>f</TD></TR>
                  </TABLE>
            </TD>
            <TD BGCOLOR="white">penguin</TD>
            </TR>
            <TR>
            <TD COLSPAN="2" BORDER="4" ALIGN="right" PORT="there">4</TD>
            </TR>
            </TABLE>>
            ]
        c [ label=<long line 1<BR/>line 2<BR ALIGN="LEFT"/>line 3<BR ALIGN="RIGHT"/>> ]
        d [ label=<<tr><td>address_id: int</td></tr>> ]
       }
        "#,
        )
            .unwrap();

        assert_eq!(
            g,
            graph!(di id!("G");
                node!("a"; attr!("label",html r#"< <TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
                    <TR><TD ROWSPAN="3" BGCOLOR="yellow">class</TD></TR>
                    <TR><TD PORT="here" BGCOLOR="lightblue">qualifier</TD></TR>
                    </TABLE>>"#)),
                node!("b";
                        attr!("shape","ellipse"),
                        attr!("style","filled"),
                        attr!("label",html r#"<
            <TABLE BGCOLOR="bisque">
            <TR>
                <TD COLSPAN="3">elephant</TD>
                <TD ROWSPAN="2" BGCOLOR="chartreuse"
                VALIGN="bottom" ALIGN="right">two</TD>
            </TR>
            <TR>
                <TD COLSPAN="2" ROWSPAN="2">
                    <TABLE BGCOLOR="grey">
                        <TR><TD>corn</TD></TR>
                        <TR><TD BGCOLOR="yellow">c</TD></TR>
                        <TR><TD>f</TD></TR>
                  </TABLE>
            </TD>
            <TD BGCOLOR="white">penguin</TD>
            </TR>
            <TR>
            <TD COLSPAN="2" BORDER="4" ALIGN="right" PORT="there">4</TD>
            </TR>
            </TABLE>>"#)
                ),
               node!("c"; attr!("label", html r#"<long line 1<BR/>line 2<BR ALIGN="LEFT"/>line 3<BR ALIGN="RIGHT"/>>"#)),
               node!("d"; attr!("label", html r#"<<tr><td>address_id: int</td></tr>>"#))

            )
        )
    }

    #[test]
    fn graph_test() {
        let g: Graph = parse(
            r#"
        strict digraph t {
            aa[color=green,label="shouln't er\ror"]
            subgraph v {
                aa[shape=square]
                subgraph vv{a2 -> b2}
                aaa[color=red]
                aaa -> bbb
            }
            aa -> be -> subgraph v { d -> aaa}
            aa -> aaa -> v
        }
        "#,
        )
            .unwrap();

        assert_eq!(
            g,
            graph!(strict di id!("t");
              node!("aa";attr!("color","green"),attr!("label", esc "shouln't er\\ror")),
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
    fn global_attr_test() {
        let g: Graph = parse(
            r#"
        graph t {
            graph [_draw_="c 9 "];
            node [label="\N"];
            a -- b;
        }
        "#,
        )
            .unwrap();

        assert_eq!(
            g,
            graph!(id!("t");
                stmt!(GraphAttributes::Graph(vec![attr!("_draw_", esc "c 9 ")])),
                stmt!(GraphAttributes::Node(vec![attr!("label", esc "\\N")])),
                edge!(node_id!("a") => node_id!("b"))
            )
        )
    }

    #[test]
    fn comments_test() {
        let g: Graph = parse("// abc \n # abc \n strict digraph t { \n /* \n abc */ \n}").unwrap();

        assert_eq!(g, graph!(strict di id!("t")))
    }

    #[test]
    fn comments_after_graph_test() {
        let g: Graph = parse("// b \n strict digraph t { \n /* \n abc */ \n} \n // a ").unwrap();

        assert_eq!(g, graph!(strict di id!("t")))
    }

    #[test]
    fn port_test() {
        let g = parse(r#"
        digraph test { A:s0 -> B;}"#).unwrap();
        assert_eq!(g, graph!(di id!("test"); edge!(node_id!("A", port!(id!("s0"))) => node_id!("B"))))
    }

    #[test]
    fn port_w_test() {
        let g = parse(r#"
        digraph test { A:s0:s -> B;}"#).unwrap();
        assert_eq!(g, graph!(di id!("test"); edge!(node_id!("A", port!(id!("s0"),"s")) => node_id!("B"))))
    }

    #[test]
    fn port_compass_test() {
        let g = parse(r#"
        digraph test { A:s -> B;}"#).unwrap();
        assert_eq!(g, graph!(di id!("test"); edge!(node_id!("A", port!(,"s")) => node_id!("B"))))
    }

    #[test]
    fn unicode_ids_and_dot_numerals() {
        assert_eq!(process_id(_parse("été", Rule::id)), id!("été"));
        assert_eq!(process_id(_parse("节点_1", Rule::id)), id!("节点_1"));
        for numeral in ["1.", "-.5", "007", "-1.25"] {
            assert_eq!(process_id(_parse(numeral, Rule::id)), id!(numeral));
        }
    }

    #[test]
    fn quoted_ids_concatenate_and_join_line_continuations() {
        assert_eq!(
            process_id(_parse(r#""ab" + "cd""#, Rule::id)),
            id!(esc "abcd")
        );
        assert_eq!(
            process_id(_parse("\"ab\\\ncd\" + \"e\\\r\nf\"", Rule::id)),
            id!(esc "abcdef")
        );
        // Other escapes are kept for the attribute that interprets them.
        assert_eq!(
            process_id(_parse(r#""a\lb""#, Rule::id)),
            id!(esc r#"a\lb"#)
        );
    }

    #[test]
    fn html_ids_may_be_single_level_or_nested() {
        assert_eq!(process_id(_parse("<abc>", Rule::id)), id!(html "<abc>"));
        assert_eq!(
            process_id(_parse("<<b>x &gt; y</b>>", Rule::id)),
            id!(html "<<b>x &gt; y</b>>")
        );
        assert_eq!(
            process_id(_parse(r#"<<td title="a>b">x</td>>"#, Rule::id)),
            id!(html r#"<<td title="a>b">x</td>>"#)
        );
    }

    #[test]
    fn keywords_are_case_insensitive_and_need_a_word_boundary() {
        let g = parse("DiGraph G { NODE [shape=box]; SubGraph s { a } }").unwrap();
        assert!(matches!(g, Graph::DiGraph { .. }));
        assert!(matches!(
            process_stmt(_parse("nodes", Rule::stmt)),
            Stmt::Node(_)
        ));
        assert!(matches!(
            process_stmt(_parse("edge_x", Rule::stmt)),
            Stmt::Node(_)
        ));
        assert!(matches!(
            process_stmt(_parse("Edge [color=red]", Rule::stmt)),
            Stmt::GAttribute(GraphAttributes::Edge(_))
        ));
    }

    #[test]
    fn compass_points_may_be_any_id() {
        assert_eq!(
            process_node_id(_parse(r#"a:p:"ne""#, Rule::node_id)),
            NodeId(id!("a"), Some(Port(Some(id!("p")), Some(r#""ne""#.to_string()))))
        );
        assert!(parse("graph { a:p:_ -- b:q:c }").is_ok());
    }

    #[test]
    fn anonymous_subgraphs_are_statements_and_edge_endpoints() {
        assert!(matches!(
            process_stmt(_parse("{ a b }", Rule::stmt)),
            Stmt::Subgraph(_)
        ));
        assert!(matches!(
            process_vertex(_parse("{ a b }", Rule::vertex)),
            Vertex::S(_)
        ));
        assert!(parse("graph { {a b} -- c; { d } -- subgraph s { e } }").is_ok());
    }

    #[test]
    fn carriage_returns_are_whitespace() {
        assert!(parse("graph {\ra -- b\r}").is_ok());
    }
}

