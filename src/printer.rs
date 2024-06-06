//! Serialize a [Graph] into a string according to the [`graphviz` DOT language].
//!
//! # Example:
//! ```rust
//!     use dot_generator::*;
//!     use dot_structures::*;
//!     use graphviz_rust::printer::{PrinterContext,DotPrinter};
//!         let mut ctx = PrinterContext::default();
//!         let s = subgraph!("id"; node!("abc"), edge!(node_id!("a") => node_id!("b")));
//!         assert_eq!(s.print(&mut ctx), "subgraph id {\n  abc\n  a -- b\n}".to_string());
//! ```
//!
//! [`graphviz` DOT language]: https://graphviz.org/doc/info/lang.html
use std::collections::HashMap;

use dot_structures::{
    Attribute, Edge, EdgeTy, Graph, GraphAttributes, Id, Node, NodeId, Port, Stmt, Subgraph, Vertex,
};

/// A function which can be passed to the [PrinterContext] to provide custom printing for attribute values.
///
/// # Example:
/// ```rust
/// use dot_generator::*;
/// use dot_structures::*;
/// use graphviz_rust::printer::{AttributeValuePrinter, DotPrinter, PrinterContext};
/// fn attr_formatter_test() {
///     let mut ctx = PrinterContext::default();
///     let formatter: Box<AttributeValuePrinter> =
///         Box::new(|value, _line_sep, _indent, _indent_step| format!(r#""**{}**""#, value.trim_matches('"')));
///     let g = graph!(di id!("attr_formatting");
///          node!("abc";attr!("custom",esc "Custom Text")),
///          edge!(node_id!("a") => node_id!("b"))
///     );
///     assert_eq!(
///         "digraph attr_formatting {\n  abc[custom=\"**Custom Text**\"]\n  a -> b\n}",
///         g.print(
///             ctx.with_node_mult_attr_s_l()
///                 .with_attr_value_printer(id!("custom"), formatter)
///         )
///     );
/// }
/// ```
pub type AttributeValuePrinter = dyn Fn(&str, &str, &str, usize) -> String;

/// Context allows to customize the output of the file.
///
/// # Example:
/// ```rust
///     use self::graphviz_rust::printer::PrinterContext;
///
///     let mut ctx = PrinterContext::default();
///     ctx.always_inline();
///     ctx.with_indent_step(4);
/// ```
pub struct PrinterContext {
    /// internal flag which is decoupled from the graph
    is_digraph: bool,
    /// a flag adds a semicolon at the end of the line
    semi: bool,
    /// a flag, print multiple node attributes on seperate lines
    mult_node_attr_on_s_l: bool,
    /// an initial indent. 0 by default
    indent: usize,
    /// a step of the indent. 2 by default
    indent_step: usize,
    /// a line separator. can be empty
    l_s: String,
    /// a len of the text to keep on one line
    inline_size: usize,
    l_s_i: String,
    l_s_m: String,
    /// a map of attribute id to AttributeValuePrinters
    attr_value_printers: HashMap<Id, Box<AttributeValuePrinter>>,
}

impl PrinterContext {
    /// Print everything on one line.
    pub fn always_inline(&mut self) -> &mut PrinterContext {
        self.l_s_m = self.l_s_i.clone();
        self.l_s = self.l_s_i.clone();
        self
    }
    /// Add a semicolon at the end of every line.
    pub fn with_semi(&mut self) -> &mut PrinterContext {
        self.semi = true;
        self
    }
    /// Print multiple attributes on seperate lines
    pub fn with_node_mult_attr_s_l(&mut self) -> &mut PrinterContext {
        self.mult_node_attr_on_s_l = true;
        self
    }
    /// Set a step of the indent.
    pub fn with_indent_step(&mut self, step: usize) -> &mut PrinterContext {
        self.indent_step = step;
        self
    }
    /// Set a specific line separator.
    pub fn with_line_sep(&mut self, sep: String) -> &mut PrinterContext {
        self.l_s = sep.clone();
        self.l_s_m = sep;
        self
    }
    /// Set the max line length.
    ///
    /// The default value is 90.
    pub fn with_inline_size(&mut self, inline_s: usize) -> &mut PrinterContext {
        self.inline_size = inline_s;
        self
    }
    /// Add an attribute printer for a specific attribute id.
    pub fn with_attr_value_printer(
        &mut self,
        attr_id: Id,
        fmt: Box<AttributeValuePrinter>,
    ) -> &mut PrinterContext {
        self.attr_value_printers.insert(attr_id, fmt);
        self
    }

    pub fn new(semi: bool, indent_step: usize, line_s: String, inline_size: usize) -> Self {
        PrinterContext {
            is_digraph: false,
            semi,
            mult_node_attr_on_s_l: false,
            indent: 0,
            indent_step,
            inline_size,
            l_s: line_s.clone(),
            l_s_i: line_s,
            l_s_m: "".to_string(),
            attr_value_printers: HashMap::new(),
        }
    }
}

impl PrinterContext {
    fn indent(&self) -> String {
        if self.is_inline_on() {
            "".to_string()
        } else {
            " ".repeat(self.indent)
        }
    }
    fn indent_grow(&mut self) {
        if !self.is_inline_on() {
            self.indent += self.indent_step
        }
    }
    fn indent_shrink(&mut self) {
        if !self.is_inline_on() {
            self.indent -= self.indent_step
        }
    }

    fn is_inline_on(&self) -> bool {
        self.l_s == self.l_s_i
    }
    fn inline_mode(&mut self) {
        self.l_s = self.l_s_i.clone()
    }
    fn multiline_mode(&mut self) {
        self.l_s = self.l_s_m.clone()
    }
}

impl Default for PrinterContext {
    fn default() -> Self {
        PrinterContext {
            is_digraph: false,
            mult_node_attr_on_s_l: false,
            semi: false,
            indent: 0,
            indent_step: 2,
            l_s: "\n".to_string(),
            inline_size: 90,
            l_s_i: "".to_string(),
            l_s_m: "\n".to_string(),
            attr_value_printers: HashMap::new(),
        }
    }
}

/// The trait for serailizing a [Graph] into the `graphviz` DOT language:
///
/// # Example:
///  ```rust
///         use dot_generator::*;
///         use dot_structures::*;
///         use self::graphviz_rust::printer::PrinterContext;
///         use self::graphviz_rust::printer::DotPrinter;
///
///         let mut ctx =PrinterContext::default();
///         ctx.always_inline();
///         ctx.with_indent_step(4);
///         let graph = graph!(strict di id!("t"));
///
///         let string = graph.print(&mut ctx);
/// ```
pub trait DotPrinter {
    fn print(&self, ctx: &mut PrinterContext) -> String;
}

impl DotPrinter for Id {
    fn print(&self, _ctx: &mut PrinterContext) -> String {
        match self {
            Id::Html(v) | Id::Escaped(v) | Id::Plain(v) => v.clone(),
            Id::Anonymous(_) => "".to_string(),
        }
    }
}

impl DotPrinter for Port {
    fn print(&self, ctx: &mut PrinterContext) -> String {
        match self {
            Port(Some(id), Some(d)) => format!(":{}:{}", id.print(ctx), d),
            Port(None, Some(d)) => format!(":{}", d),
            Port(Some(id), None) => format!(":{}", id.print(ctx)),
            _ => unreachable!(""),
        }
    }
}

impl DotPrinter for NodeId {
    fn print(&self, ctx: &mut PrinterContext) -> String {
        match self {
            NodeId(id, None) => id.print(ctx),
            NodeId(id, Some(port)) => [id.print(ctx), port.print(ctx)].join(""),
        }
    }
}

impl DotPrinter for Attribute {
    fn print(&self, ctx: &mut PrinterContext) -> String {
        match self {
            Attribute(l, r) => {
                let l_val = l.print(ctx);
                let r_val = r.print(ctx);
                if let Some(formatter) = ctx.attr_value_printers.get(l) {
                    format!(
                        "{}={}",
                        l_val,
                        formatter(&r_val, &ctx.l_s, &ctx.indent(), ctx.indent_step)
                    )
                } else {
                    format!("{}={}", l_val, r_val)
                }
            }
        }
    }
}

impl DotPrinter for Vec<Attribute> {
    fn print(&self, ctx: &mut PrinterContext) -> String {
        let attrs: Vec<String> = self.iter().map(|e| e.print(ctx)).collect();
        if attrs.is_empty() {
            "".to_string()
        } else if attrs.len() > 1 && ctx.mult_node_attr_on_s_l {
            let indent = ctx.indent();
            ctx.indent_grow();
            let r = format!(
                "[{}{}{}{}{}]",
                ctx.l_s,
                ctx.indent(),
                attrs.join(&format!(",{}{}", ctx.l_s, ctx.indent())),
                ctx.l_s,
                indent,
            );
            ctx.indent_shrink();
            r
        } else {
            format!("[{}]", attrs.join(","))
        }
    }
}

impl DotPrinter for GraphAttributes {
    fn print(&self, ctx: &mut PrinterContext) -> String {
        match self {
            GraphAttributes::Graph(attrs) => format!("graph{}", attrs.print(ctx)),
            GraphAttributes::Node(attrs) => format!("node{}", attrs.print(ctx)),
            GraphAttributes::Edge(attrs) => format!("edge{}", attrs.print(ctx)),
        }
    }
}

impl DotPrinter for Node {
    fn print(&self, ctx: &mut PrinterContext) -> String {
        format!("{}{}", self.id.print(ctx), self.attributes.print(ctx))
    }
}

impl DotPrinter for Vertex {
    fn print(&self, ctx: &mut PrinterContext) -> String {
        match self {
            Vertex::N(el) => el.print(ctx),
            Vertex::S(el) => el.print(ctx),
        }
    }
}

impl DotPrinter for Subgraph {
    fn print(&self, ctx: &mut PrinterContext) -> String {
        let indent = ctx.indent();
        ctx.indent_grow();
        let header = format!("subgraph {} {{{}", self.id.print(ctx), ctx.l_s);
        let r = format!("{}{}{}{}}}", header, self.stmts.print(ctx), ctx.l_s, indent);
        ctx.indent_shrink();
        r
    }
}

impl DotPrinter for Graph {
    fn print(&self, ctx: &mut PrinterContext) -> String {
        ctx.indent_grow();

        match self {
            Graph::Graph { id, strict, stmts } if *strict => {
                ctx.is_digraph = false;
                let body = stmts.print(ctx);
                format!(
                    "strict graph {} {{{}{}{}}}",
                    id.print(ctx),
                    ctx.l_s,
                    body,
                    ctx.l_s
                )
            }
            Graph::Graph {
                id,
                strict: _,
                stmts,
            } => {
                ctx.is_digraph = false;
                let body = stmts.print(ctx);
                format!("graph {} {{{}{}{}}}", id.print(ctx), ctx.l_s, body, ctx.l_s)
            }
            Graph::DiGraph { id, strict, stmts } if *strict => {
                ctx.is_digraph = true;
                let body = stmts.print(ctx);
                format!(
                    "strict digraph {} {{{}{}{}}}",
                    id.print(ctx),
                    ctx.l_s,
                    body,
                    ctx.l_s
                )
            }
            Graph::DiGraph {
                id,
                strict: _,
                stmts,
            } => {
                ctx.is_digraph = true;
                let body = stmts.print(ctx);
                format!(
                    "digraph {} {{{}{}{}}}",
                    id.print(ctx),
                    ctx.l_s,
                    body,
                    ctx.l_s
                )
            }
        }
    }
}

impl DotPrinter for Vec<Stmt> {
    fn print(&self, ctx: &mut PrinterContext) -> String {
        let attrs: Vec<String> = self.iter().map(|e| e.print(ctx)).collect();
        attrs.join(ctx.l_s.as_str())
    }
}

impl DotPrinter for Stmt {
    fn print(&self, ctx: &mut PrinterContext) -> String {
        let end = if ctx.semi { ";" } else { "" };
        let indent = ctx.indent();
        match self {
            Stmt::Node(e) => format!("{}{}{}", indent, e.print(ctx), end),
            Stmt::Subgraph(e) => format!("{}{}{}", indent, e.print(ctx), end),
            Stmt::Attribute(e) => format!("{}{}{}", indent, e.print(ctx), end),
            Stmt::GAttribute(e) => format!("{}{}{}", indent, e.print(ctx), end),
            Stmt::Edge(e) => format!("{}{}{}", indent, e.print(ctx), end),
        }
    }
}

fn print_edge(edge: &Edge, ctx: &mut PrinterContext) -> String {
    let bond = if ctx.is_digraph { "->" } else { "--" };
    match edge {
        Edge {
            ty: EdgeTy::Pair(l, r),
            attributes,
        } => {
            if attributes.is_empty() {
                format!("{} {} {}", l.print(ctx), bond, r.print(ctx))
            } else {
                format!(
                    "{} {} {} {}",
                    l.print(ctx),
                    bond,
                    r.print(ctx),
                    attributes.print(ctx)
                )
            }
        }
        Edge {
            ty: EdgeTy::Chain(vs),
            attributes,
        } => {
            let mut iter = vs.iter();
            let h = iter.next().unwrap().print(ctx);
            let mut chain = h;
            for el in iter {
                chain = format!("{} {} {}", chain, bond, el.print(ctx))
            }
            format!("{}{}", chain, attributes.print(ctx))
        }
    }
}

impl DotPrinter for Edge {
    fn print(&self, ctx: &mut PrinterContext) -> String {
        let mut edge_str = print_edge(self, ctx);
        if edge_str.len() <= ctx.inline_size && !ctx.is_inline_on() {
            ctx.inline_mode();
            edge_str = print_edge(self, ctx);
            ctx.multiline_mode();
        }

        edge_str
    }
}

#[cfg(test)]
mod tests {
    use dot_generator::{attr, edge, graph, id, node, node_id, port, stmt, subgraph};
    use dot_structures::*;

    use crate::printer::{DotPrinter, PrinterContext};

    #[test]
    fn edge_test() {
        let mut ctx = PrinterContext::default();
        let edge = edge!(node_id!("abc") => node_id!("bce") => node_id!("cde"); attr!("a",2));
        assert_eq!(edge.print(&mut ctx), "abc -- bce -- cde[a=2]");
        ctx.is_digraph = true;
        assert_eq!(edge.print(&mut ctx), "abc -> bce -> cde[a=2]");
    }

    #[test]
    fn node_id_test() {
        let node_id = NodeId(id!("abc"), Some(port!(id!("abc"), "n")));
        let mut ctx = PrinterContext::default();
        assert_eq!(node_id.print(&mut ctx), "abc:abc:n".to_string());
    }

    #[test]
    fn node_test() {
        let mut ctx = PrinterContext::default();
        assert_eq!(
            node!("abc";attr!("a",2)).print(&mut ctx),
            "abc[a=2]".to_string()
        );
    }

    #[test]
    fn attr_test() {
        let mut ctx = PrinterContext::default();
        let attr = attr!("a", 2);
        assert_eq!(attr.print(&mut ctx), "a=2".to_string());
    }

    #[test]
    fn graph_attr_test() {
        let mut ctx = PrinterContext::default();
        let n_attr = GraphAttributes::Node(vec![attr!("a", 2), attr!("b", 3)]);
        assert_eq!(n_attr.print(&mut ctx), "node[a=2,b=3]".to_string());
    }

    #[test]
    fn subgraph_test() {
        let mut ctx = PrinterContext::default();
        let s = subgraph!("id"; node!("abc"), edge!(node_id!("a") => node_id!("b")));
        println!("{}", s.print(&mut ctx));
        assert_eq!(
            s.print(&mut ctx),
            "subgraph id {\n  abc\n  a -- b\n}".to_string()
        );
    }

    #[test]
    fn graph_test() {
        let mut ctx = PrinterContext::default();
        ctx.always_inline();
        let g = graph!(strict di id!("t");
          node!("aa";attr!("color","green")),
          subgraph!("v";
            node!("aa"; attr!("shape","square")),
            subgraph!("vv"; edge!(node_id!("a2") => node_id!("b2"))),
            node!("aaa";attr!("color","red")),
            edge!(node_id!("aaa") => node_id!("bbb"))
            ),
          edge!(node_id!("aa") => node_id!("be") => subgraph!("v"; edge!(node_id!("d") => node_id!("aaa")))),
          edge!(node_id!("aa") => node_id!("aaa") => node_id!("v"))
        );
        assert_eq!(
            r#"strict digraph t {aa[color=green]subgraph v {aa[shape=square]subgraph vv {a2 -> b2}aaa[color=red]aaa -> bbb}aa -> be -> subgraph v {d -> aaa}aa -> aaa -> v}"#,
            g.print(&mut ctx)
        );
    }

    #[test]
    fn semi_graph_test() {
        let mut ctx = PrinterContext::default();
        let g = graph!(strict di id!("t");
          node!("aa";attr!("color","green")),
          subgraph!("v";
            node!("aa"; attr!("shape","square")),
            subgraph!("vv"; edge!(node_id!("a2") => node_id!("b2"))),
            node!("aaa";attr!("color","red")),
            edge!(node_id!("aaa") => node_id!("bbb"))
            ),
          edge!(node_id!("aa") => node_id!("be") => subgraph!("v"; edge!(node_id!("d") => node_id!("aaa")))),
          edge!(node_id!("aa") => node_id!("aaa") => node_id!("v"))
        );
        assert_eq!(
            "strict digraph t {\n  aa[color=green];\n  subgraph v {\n    aa[shape=square];\n    subgraph vv {\n      a2 -> b2;\n    };\n    aaa[color=red];\n    aaa -> bbb;\n  };\n  aa -> be -> subgraph v {d -> aaa;};\n  aa -> aaa -> v;\n}",
            g.print(ctx.with_semi())
        );
    }

    #[test]
    fn indent_step_graph_test() {
        let mut ctx = PrinterContext::default();
        let g = graph!(strict di id!("t");
          node!("aa";attr!("color","green")),
          subgraph!("v";
            node!("aa"; attr!("shape","square")),
            subgraph!("vv"; edge!(node_id!("a2") => node_id!("b2"))),
            node!("aaa";attr!("color","red")),
            edge!(node_id!("aaa") => node_id!("bbb"))
            ),
          edge!(node_id!("aa") => node_id!("be") => subgraph!("v"; edge!(node_id!("d") => node_id!("aaa")))),
          edge!(node_id!("aa") => node_id!("aaa") => node_id!("v"))
        );
        assert_eq!(
            "strict digraph t {\n    aa[color=green]\n    subgraph v {\n        aa[shape=square]\n        subgraph vv {\n            a2 -> b2\n        }\n        aaa[color=red]\n        aaa -> bbb\n    }\n    aa -> be -> subgraph v {d -> aaa}\n    aa -> aaa -> v\n}",
            g.print(ctx.with_indent_step(4))
        );
    }

    #[test]
    fn mult_attr_l_s_graph_test() {
        let mut ctx = PrinterContext::default();
        let g = graph!(di id!("multi");
          node!("a";attr!("shape","square")),
          node!("aa";attr!("color","blue"),attr!("shape","Mrecord")),
          subgraph!("v";
            node!("aaa"; attr!("shape","square")),
            node!("aaaa";attr!("color","red"),attr!("shape","Mrecord")),
            edge!(node_id!("aaa") => node_id!("aaaa");attr!("label","FALSE"))
          ),
          edge!(node_id!("a") => node_id!("aa");attr!("label","TRUE"), attr!("color","green"))
        );
        assert_eq!(
            "digraph multi {\n  a[shape=square]\n  aa[\n    color=blue,\n    shape=Mrecord\n  ]\n  subgraph v {\n    aaa[shape=square]\n    aaaa[\n      color=red,\n      shape=Mrecord\n    ]\n    aaa -> aaaa [label=FALSE]\n  }\n  a -> aa [label=TRUE,color=green]\n}",
            g.print(ctx.with_node_mult_attr_s_l())
        );
    }

    #[test]
    fn attr_formatter_graph_test() {
        let mut ctx = PrinterContext::default();
        let g = graph!(di id!("multi");
          node!("a";attr!("shape","square")),
          node!("aa";attr!("color","blue"),attr!("custom", esc "Custom Text"),attr!("shape","Mrecord")),
          subgraph!("v";
            node!("aaa"; attr!("shape","square")),
            node!("aaaa";attr!("color","red"),attr!("custom", esc "Custom Text2")),
            edge!(node_id!("aaa") => node_id!("aaaa");attr!("label","FALSE"))
          ),
          edge!(node_id!("a") => node_id!("aa");attr!("label","TRUE"), attr!("color","green"))
        );
        assert_eq!(
            "digraph multi {\n  a[shape=square]\n  aa[\n    color=blue,\n    custom=\"**Custom Text**\",\n    shape=Mrecord\n  ]\n  subgraph v {\n    aaa[shape=square]\n    aaaa[\n      color=red,\n      custom=\"**Custom Text2**\"\n    ]\n    aaa -> aaaa [label=FALSE]\n  }\n  a -> aa [label=TRUE,color=green]\n}",
            g.print(ctx.with_node_mult_attr_s_l().with_attr_value_printer(id!("custom"), Box::new(|value, _l_s, _indent, _i_s| {
                format!(r#""**{}**""#, value.trim_matches('"'))
            })))
        );
    }
}
