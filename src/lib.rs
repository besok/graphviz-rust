//! The library allows to interact with [`graphviz`] format.
//!
//! # Description:
//! Essentially, it starts from 3 base methods:
//!  - parse: a source of a dot file in the dot [`notation`]. The output format is a [Graph].
//!  - print: [Graph] and [DotPrinter] provides an ability to transform a graph into string
//!         following the [`notation`]
//!  - exec: an ability to [`execute`] a cmd graphviz engine into different formats and etc.
//!  - exec_dot: an ability to [`execute`] a cmd graphviz engine into different formats from a prepared string containing a dot graph.
//!
//! # Examples:
//! ```rust
//! use dot_generator::*;
//! use dot_structures::*;
//! use graphviz_rust::{
//!     attributes::*,
//!     cmd::{CommandArg, Format},
//!     exec, exec_dot, parse,
//!     printer::{DotPrinter, PrinterContext},
//! };
//!
//! fn parse_test() {
//!     let g: Graph = parse(
//!         r#"
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
//!         "#,
//!     )
//!     .unwrap();
//!
//!     assert_eq!(
//!         g,
//!         graph!(strict di id!("t");
//!           node!("aa";attr!("color","green")),
//!           subgraph!("v";
//!             node!("aa"; attr!("shape","square")),
//!             subgraph!("vv"; edge!(node_id!("a2") => node_id!("b2"))),
//!             node!("aaa";attr!("color","red")),
//!             edge!(node_id!("aaa") => node_id!("bbb"))
//!             ),
//!           edge!(node_id!("aa") => node_id!("be") => subgraph!("v"; edge!(node_id!("d") => node_id!("aaa")))),
//!           edge!(node_id!("aa") => node_id!("aaa") => node_id!("v"))
//!         )
//!     )
//! }
//!
//! fn print_test() {
//!     let mut g = graph!(strict di id!("id"));
//!     assert_eq!(
//!         "strict digraph id {}".to_string(),
//!         g.print(&mut PrinterContext::default())
//!     );
//! }
//!
//! fn output_test() {
//!     let mut g = graph!(id!("id");
//!          node!("nod"),
//!          subgraph!("sb";
//!              edge!(node_id!("a") => subgraph!(;
//!                 node!("n";
//!                 NodeAttributes::color(color_name::black), NodeAttributes::shape(shape::egg))
//!             ))
//!         ),
//!         edge!(node_id!("a1") => node_id!(esc "a2"))
//!     );
//!     let graph_svg = exec(
//!         g,
//!         &mut PrinterContext::default(),
//!         vec![CommandArg::Format(Format::Svg)],
//!     )
//!     .unwrap();
//! }
//! fn output_exec_from_test() {
//!     let mut g = graph!(id!("id");
//!          node!("nod"),
//!          subgraph!("sb";
//!              edge!(node_id!("a") => subgraph!(;
//!                 node!("n";
//!                 NodeAttributes::color(color_name::black), NodeAttributes::shape(shape::egg))
//!             ))
//!         ),
//!         edge!(node_id!("a1") => node_id!(esc "a2"))
//!     );
//!     let dot = g.print(&mut PrinterContext::default());
//!     println!("{}", dot);
//!     let format = Format::Svg;
//!
//!     let graph_svg = exec_dot(dot.clone(), vec![CommandArg::Format(format)]).unwrap();
//!
//!     let graph_svg = exec_dot(dot, vec![CommandArg::Format(format.clone())]).unwrap();
//! }
//! ```
//!
//! [`graphviz`]: https://graphviz.org/
//! [`notation`]: https://graphviz.org/doc/info/lang.html
//! [`execute`]: https://graphviz.org/doc/info/command.html
#![allow(non_camel_case_types)]
#![allow(dead_code)]
pub extern crate dot_generator;
pub extern crate dot_structures;
pub extern crate into_attr;
pub extern crate into_attr_derive;

use std::io;

use dot_structures::*;

use crate::{
    cmd::CommandArg,
    printer::{DotPrinter, PrinterContext},
};

pub mod attributes;
pub mod cmd;
mod parser;
pub mod printer;

#[macro_use]
extern crate pest_derive;
extern crate pest;

/// Parses a given string into a graph format that can be used afterwards or returning
/// an string with an error description
pub fn parse(dot: &str) -> Result<Graph, String> {
    parser::parse(dot)
}

/// Prints a given graph according to a given [`PrinterContext`]
pub fn print(graph: Graph, ctx: &mut PrinterContext) -> String {
    graph.print(ctx)
}

/// Executes a given graph using a dot cmd client
pub fn exec(graph: Graph, ctx: &mut PrinterContext, args: Vec<CommandArg>) -> io::Result<String> {
    cmd::exec(print(graph, ctx), args)
}

/// Executes a given string representation of the graph using a dot cmd client
pub fn exec_dot(dot_graph: String, args: Vec<CommandArg>) -> io::Result<String> {
    cmd::exec(dot_graph, args)
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::{Path, PathBuf},
        process::Command,
    };

    use dot_generator::*;
    use dot_structures::*;

    use crate::{
        attributes::*,
        cmd::{CommandArg, Format},
        exec, exec_dot, parse,
        printer::{DotPrinter, PrinterContext},
    };

    #[test]
    fn parse_test() {
        let g: Graph = parse(
            r#"
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
        "#,
        )
        .unwrap();

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
    fn print_test() {
        let mut g = graph!(id!("id"));

        for el in (1..10000).into_iter() {
            if el % 2 == 0 {
                g.add_stmt(stmt!(node!(el)))
            } else {
                g.add_stmt(stmt!(subgraph!(el)))
            }
        }

        assert_eq!(178896, g.print(&mut PrinterContext::default()).len())
    }

    #[cfg(windows)]
    const LS: &'static str = "\r\n";
    #[cfg(not(windows))]
    const LS: &'static str = "\n";

    #[test]
    fn exec_test() {
        let mut g = graph!(id!("id");
            node!("nod"),
            subgraph!("sb";
                edge!(node_id!("a") => subgraph!(;
                    node!("n";
                    NodeAttributes::color(color_name::black), NodeAttributes::shape(shape::egg))
                ))
            ),
            edge!(node_id!("a1") => node_id!(esc "a2"))
        );
        let graph_str = "graph id {\n    nod\n    subgraph sb {\n        a -- subgraph  {n[color=black,shape=egg]} \n    }\n    a1 -- \"a2\" \n}";

        let mut ctx = PrinterContext::default();
        assert_eq!(graph_str, g.print(&mut ctx));

        let child = Command::new("dot")
            .arg("-V")
            .output()
            .expect("dot command failed to start");

        let output = String::from_utf8_lossy(&child.stderr);
        let version = output
            .strip_prefix("dot - ")
            .and_then(|v| v.strip_suffix(LS))
            .expect("the version of client is unrecognizable ");
        println!("{}", version);

        let out_svg = exec(g.clone(), &mut ctx, vec![CommandArg::Format(Format::Svg)]).unwrap();

        let p = "1.svg";
        let out = exec(
            g.clone(),
            &mut PrinterContext::default(),
            vec![
                CommandArg::Format(Format::Svg),
                CommandArg::Output(p.to_string()),
            ],
        )
        .unwrap();

        let file = fs::read_to_string(p).unwrap();

        fs::remove_file(p).unwrap();
        assert_eq!("", out);
        assert_eq!(out_svg, file);
    }

    #[test]
    fn output_exec_from_test() {
        let mut g = graph!(id!("id");
             node!("nod"),
             subgraph!("sb";
                 edge!(node_id!("a") => subgraph!(;
                    node!("n";
                    NodeAttributes::color(color_name::black), NodeAttributes::shape(shape::egg))
                ))
            ),
            edge!(node_id!("a1") => node_id!(esc "a2"))
        );
        let dot = g.print(&mut PrinterContext::default());
        let format = Format::Svg;

        let res1 = exec_dot(dot.clone(), vec![CommandArg::Format(format)]).unwrap();
        let res2 = exec_dot(dot.clone(), vec![CommandArg::Format(format.clone())]).unwrap();

        assert_eq!(res1, res2)
    }
}
