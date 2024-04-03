//! The library provides functionality for interacting with the [`graphviz` DOT language].
//!
//! # Description:
//! This library contains 4 primary functions:
//!  - [parse]: parses a string in the dot [`notation`] into a [Graph].
//!  - [print](crate::print): serializes a [Graph] into a string given a [DotPrinter].
//!  - [exec]: executes the [`dot` command line executable] given a [Graph].
//!  - [exec_dot]: executes the [`dot` command line executable] given a string in
//!    the dot [`notation`].
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
//!     );
//!
//!     let mut g = graph!(strict di id!("id"));
//!     assert_eq!(
//!         "strict digraph id {\n\n}".to_string(),
//!         g.print(&mut PrinterContext::default())
//!     );
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
//!         vec![Format::Svg.into()],
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
//!     let graph_svg = exec_dot(dot.clone(), vec![format.into()]).unwrap();
//!
//!     let graph_svg = exec_dot(dot, vec![format.clone().into()]).unwrap();
//! }
//! ```
//!
//! [`graphviz`]: https://graphviz.org/
//! [`graphviz` DOT language]: https://graphviz.org/doc/info/lang.html
//! [`notation`]: https://graphviz.org/doc/info/lang.html
//! [`dot` command line executable]: https://graphviz.org/doc/info/command.html
#![allow(non_camel_case_types)]
#![allow(dead_code)]
pub extern crate dot_generator;
pub extern crate dot_structures;
pub extern crate into_attr;
pub extern crate into_attr_derive;

use dot_structures::*;

use crate::printer::{DotPrinter, PrinterContext};

pub mod attributes;
#[cfg(feature = "graphviz-exec")]
pub mod cmd;
mod parser;
pub mod printer;

#[macro_use]
extern crate pest_derive;
extern crate pest;

/// Parses a string into a [Graph].
pub fn parse(dot: &str) -> Result<Graph, String> {
    parser::parse(dot)
}

/// Serializes a [Graph] into a string given a [DotPrinter].
pub fn print(graph: Graph, ctx: &mut PrinterContext) -> String {
    graph.print(ctx)
}

#[cfg(feature = "graphviz-exec")]
use cmd::CommandArg;
#[cfg(feature = "graphviz-exec")]
use std::io;

/// Executes the [`dot` command line executable](https://graphviz.org/doc/info/command.html)
/// using the given [Graph], [PrinterContext] and command line arguments.
#[cfg(feature = "graphviz-exec")]
pub fn exec(graph: Graph, ctx: &mut PrinterContext, args: Vec<CommandArg>) -> io::Result<Vec<u8>> {
    cmd::exec(print(graph, ctx), args)
}

/// Executes the [`dot` command line executable](https://graphviz.org/doc/info/command.html)
/// using the given string dot notation, [PrinterContext] and command line arguments.
#[cfg(feature = "graphviz-exec")]
pub fn exec_dot(dot_graph: String, args: Vec<CommandArg>) -> io::Result<Vec<u8>> {
    cmd::exec(dot_graph, args)
}

#[cfg(test)]
mod tests {

    use dot_generator::*;
    use dot_structures::*;

    use crate::{
        parse,
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

        assert_eq!(148898, g.print(&mut PrinterContext::default()).len())
    }

    #[cfg(windows)]
    const LS: &'static str = "\r\n";
    #[cfg(not(windows))]
    const LS: &'static str = "\n";

    #[test]
    #[cfg(feature = "graphviz-exec")]
    fn exec_test() {
        use std::{fs, process::Command};

        use crate::{
            attributes::{color_name, shape, NodeAttributes},
            cmd::{CommandArg, Format},
            exec,
        };

        let g = graph!(id!("id");
            node!("nod"),
            subgraph!("sb";
                edge!(node_id!("a") => subgraph!(;
                    node!("n";
                    NodeAttributes::color(color_name::black), NodeAttributes::shape(shape::egg))
                ))
            ),
            edge!(node_id!("a1") => node_id!(esc "a2"))
        );
        let graph_str = "graph id {\n  nod\n  subgraph sb {\n    a -- subgraph  {n[color=black,shape=egg]}\n  }\n  a1 -- \"a2\"\n}";

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
        assert_eq!(Vec::<u8>::new(), out);
        assert_eq!(out_svg, file.as_bytes());
    }

    #[test]
    #[cfg(feature = "graphviz-exec")]
    fn output_exec_from_test() {
        use crate::{
            attributes::{color_name, shape, NodeAttributes},
            cmd::Format,
            exec_dot,
        };

        let g = graph!(id!("id");
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

        let res1 = exec_dot(dot.clone(), vec![format.into()]).unwrap();
        let res2 = exec_dot(dot.clone(), vec![format.clone().into()]).unwrap();

        assert_eq!(res1, res2)
    }
}
