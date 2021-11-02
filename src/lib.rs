//! The library allows to interact with [`graphviz`] format.
//!
//! # Description:
//! Essentially, it starts from 3 base methods:
//!  - parse: a source of a dot file in the dot [`notation`]. The output format is a [Graph].
//!  - print: [Graph] and [DotPrinter] provides an ability to transform a graph into string
//!         following the [`notation`]
//!  - exec: an ability to [`execute`] a cmd graphviz engine into different formats and etc.
//!
//! # Examples:
//! ```rust
//! use dot_structures::*;
//! use dot_generator::*;
//! use graphviz_rust::{exec, parse};
//! use graphviz_rust::cmd::{CommandArg, Format};
//! use graphviz_rust::printer::{PrinterContext,DotPrinter};
//! use graphviz_rust::attributes::*;
//!
//! fn parse_test() {
//!        let g: Graph = parse(r#"
//!        strict digraph t {
//!            aa[color=green]
//!            subgraph v {
//!                aa[shape=square]
//!                subgraph vv{a2 -> b2}
//!                aaa[color=red]
//!                aaa -> bbb
//!            }
//!            aa -> be -> subgraph v { d -> aaa}
//!            aa -> aaa -> v
//!        }
//!        "#).unwrap();
//!
//!        assert_eq!(
//!            g,
//!            graph!(strict di id!("t");
//!              node!("aa";attr!("color","green")),
//!              subgraph!("v";
//!                node!("aa"; attr!("shape","square")),
//!                subgraph!("vv"; edge!(node_id!("a2") => node_id!("b2"))),
//!                node!("aaa";attr!("color","red")),
//!                edge!(node_id!("aaa") => node_id!("bbb"))
//!                ),
//!              edge!(node_id!("aa") => node_id!("be") => subgraph!("v"; edge!(node_id!("d") => node_id!("aaa")))),
//!              edge!(node_id!("aa") => node_id!("aaa") => node_id!("v"))
//!            )
//!        )
//!    }
//!
//! fn print_test() {
//!        let mut g = graph!(strict di id!("id"));
//!        assert_eq!("strict digraph id {}".to_string(), g.print(&mut PrinterContext::default()));
//!    }
//!
//!  fn output_test(){
//!     let mut g = graph!(id!("id");
//!             node!("nod"),
//!             subgraph!("sb";
//!                 edge!(node_id!("a") => subgraph!(;
//!                    node!("n";
//!                    NodeAttributes::color(color_name::black), NodeAttributes::shape(shape::egg))
//!                ))
//!            ),
//!            edge!(node_id!("a1") => node_id!(esc "a2"))
//!        );
//!        let graph_svg = exec(g, PrinterContext::default(), vec![
//!            CommandArg::Format(Format::Svg),
//!        ]).unwrap();
//!
//!  }
//! ```
//!
//! [`graphviz`]: https://graphviz.org/
//! [`notation`]: https://graphviz.org/doc/info/lang.html
//! [`execute`]: https://graphviz.org/doc/info/command.html
//!
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::io;
use dot_structures::*;
use crate::cmd::CommandArg;
use crate::printer::{DotPrinter, PrinterContext};

pub mod attributes;
pub mod printer;
pub mod cmd;
mod parser;

#[macro_use]
extern crate pest_derive;
extern crate pest;

pub fn parse(dot: &str) -> Result<Graph, String> {
    parser::parse(dot)
}

pub fn print(graph: Graph, mut ctx: PrinterContext) -> String {
    graph.print(&mut ctx)
}

pub fn exec(graph: Graph, mut ctx: PrinterContext, args: Vec<CommandArg>) -> io::Result<String> {
    cmd::exec(print(graph, ctx), args)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};
    use dot_structures::*;
    use dot_generator::*;
    use crate::attributes::*;
    use crate::{exec, parse};
    use crate::cmd::{CommandArg, Format};
    use crate::printer::{DotPrinter, PrinterContext};

    #[test]
    fn parse_test() {
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
        let graph_str = "graph id {\n    nod\n    subgraph sb {\n        a -- subgraph  {n[color=black,shape=egg]} \n    }\n    a1 -- a2 \n}";
        let graph_str_svg = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\n<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\"\n \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\n<!-- Generated by graphviz version 2.49.2 (20211016.1639)\n -->\n<!-- Title: id Pages: 1 -->\n<svg width=\"208pt\" height=\"116pt\"\n viewBox=\"0.00 0.00 208.00 116.00\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">\n<g id=\"graph0\" class=\"graph\" transform=\"scale(1 1) rotate(0) translate(4 112)\">\n<title>id</title>\n<polygon fill=\"white\" stroke=\"transparent\" points=\"-4,4 -4,-112 204,-112 204,4 -4,4\"/>\n<!-- nod -->\n<g id=\"node1\" class=\"node\">\n<title>nod</title>\n<ellipse fill=\"none\" stroke=\"black\" cx=\"27\" cy=\"-90\" rx=\"27\" ry=\"18\"/>\n<text text-anchor=\"middle\" x=\"27\" y=\"-86.3\" font-family=\"Times,serif\" font-size=\"14.00\">nod</text>\n</g>\n<!-- a -->\n<g id=\"node2\" class=\"node\">\n<title>a</title>\n<ellipse fill=\"none\" stroke=\"black\" cx=\"99\" cy=\"-90\" rx=\"27\" ry=\"18\"/>\n<text text-anchor=\"middle\" x=\"99\" y=\"-86.3\" font-family=\"Times,serif\" font-size=\"14.00\">a</text>\n</g>\n<!-- n -->\n<g id=\"node3\" class=\"node\">\n<title>n</title>\n<polygon fill=\"none\" stroke=\"black\" points=\"101.68,-0.05 103.46,-0.15 105.22,-0.3 106.95,-0.49 108.65,-0.74 110.31,-1.03 111.92,-1.36 113.48,-1.75 114.99,-2.18 116.43,-2.65 117.8,-3.16 119.1,-3.71 120.32,-4.31 121.45,-4.94 122.51,-5.61 123.47,-6.31 124.35,-7.04 125.13,-7.8 125.81,-8.59 126.41,-9.41 126.91,-10.25 127.31,-11.11 127.62,-11.99 127.83,-12.89 127.96,-13.8 127.99,-14.72 127.93,-15.65 127.79,-16.59 127.57,-17.53 127.27,-18.47 126.89,-19.41 126.44,-20.35 125.92,-21.28 125.33,-22.2 124.69,-23.11 123.98,-24.01 123.22,-24.89 122.41,-25.75 121.56,-26.59 120.67,-27.41 119.73,-28.2 118.76,-28.96 117.76,-29.69 116.74,-30.39 115.68,-31.06 114.61,-31.69 113.52,-32.29 112.41,-32.84 111.28,-33.35 110.14,-33.82 108.99,-34.25 107.84,-34.64 106.67,-34.97 105.5,-35.26 104.33,-35.51 103.15,-35.7 101.96,-35.85 100.78,-35.95 99.59,-36 98.41,-36 97.22,-35.95 96.04,-35.85 94.85,-35.7 93.67,-35.51 92.5,-35.26 91.33,-34.97 90.16,-34.64 89.01,-34.25 87.86,-33.82 86.72,-33.35 85.59,-32.84 84.48,-32.29 83.39,-31.69 82.32,-31.06 81.26,-30.39 80.24,-29.69 79.24,-28.96 78.27,-28.2 77.33,-27.41 76.44,-26.59 75.59,-25.75 74.78,-24.89 74.02,-24.01 73.31,-23.11 72.67,-22.2 72.08,-21.28 71.56,-20.35 71.11,-19.41 70.73,-18.47 70.43,-17.53 70.21,-16.59 70.07,-15.65 70.01,-14.72 70.04,-13.8 70.17,-12.89 70.38,-11.99 70.69,-11.11 71.09,-10.25 71.59,-9.41 72.19,-8.59 72.87,-7.8 73.65,-7.04 74.53,-6.31 75.49,-5.61 76.55,-4.94 77.68,-4.31 78.9,-3.71 80.2,-3.16 81.57,-2.65 83.01,-2.18 84.52,-1.75 86.08,-1.36 87.69,-1.03 89.35,-0.74 91.05,-0.49 92.78,-0.3 94.54,-0.15 96.32,-0.05 98.1,0 99.9,0 101.68,-0.05\"/>\n<text text-anchor=\"middle\" x=\"99\" y=\"-14.3\" font-family=\"Times,serif\" font-size=\"14.00\">n</text>\n</g>\n<!-- a&#45;&#45;n -->\n<g id=\"edge1\" class=\"edge\">\n<title>a&#45;&#45;n</title>\n<path fill=\"none\" stroke=\"black\" d=\"M99,-71.7C99,-60.85 99,-46.92 99,-36.1\"/>\n</g>\n<!-- a1 -->\n<g id=\"node4\" class=\"node\">\n<title>a1</title>\n<ellipse fill=\"none\" stroke=\"black\" cx=\"173\" cy=\"-90\" rx=\"27\" ry=\"18\"/>\n<text text-anchor=\"middle\" x=\"173\" y=\"-86.3\" font-family=\"Times,serif\" font-size=\"14.00\">a1</text>\n</g>\n<!-- a2 -->\n<g id=\"node5\" class=\"node\">\n<title>a2</title>\n<ellipse fill=\"none\" stroke=\"black\" cx=\"173\" cy=\"-18\" rx=\"27\" ry=\"18\"/>\n<text text-anchor=\"middle\" x=\"173\" y=\"-14.3\" font-family=\"Times,serif\" font-size=\"14.00\">a2</text>\n</g>\n<!-- a1&#45;&#45;a2 -->\n<g id=\"edge2\" class=\"edge\">\n<title>a1&#45;&#45;a2</title>\n<path fill=\"none\" stroke=\"black\" d=\"M173,-71.7C173,-60.85 173,-46.92 173,-36.1\"/>\n</g>\n</g>\n</svg>\n";
        assert_eq!(graph_str, g.print(&mut PrinterContext::default()));
        let out = exec(g.clone(), PrinterContext::default(), vec![
            CommandArg::Format(Format::Svg),
        ]).unwrap();

        assert_eq!(graph_str_svg,out);

        let p = "1.svg";
        let out = exec(g.clone(), PrinterContext::default(), vec![
            CommandArg::Format(Format::Svg),
            CommandArg::Output(p.to_string())
        ]).unwrap();

        assert_eq!("",out);

        let file = fs::read_to_string(p).unwrap();
        assert_eq!(graph_str_svg,file);

        std::fs::remove_file(p);
    }
}