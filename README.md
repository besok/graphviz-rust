### Description

The library provides the basic access to the graphs in [graphviz](https://graphviz.org/) format with ability to import
into or export from it.

### Base examples

#### Parse dot source

```rust
use dot_generator::*;
use dot_structures::*;

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
```

#### Print graph into dot source

```rust
use dot_generator::*;
use dot_structures::*;
use graphviz_rust::printer::{DotPrinter, PrinterContext};

fn print_test() {
    let mut g = graph!(strict di id!("id"));
    assert_eq!(
        "strict digraph id {}".to_string(),
        g.print(&mut PrinterContext::default())
    );
}
```

#### Transform graph into external formats with cmd engine

```rust
use dot_generator::*;
use dot_structures::*;
use graphviz_rust::{
    attributes::*,
    cmd::{CommandArg, Format},
    exec, parse,
    printer::{DotPrinter, PrinterContext},
};

fn output_test() {
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
    let graph_svg = exec(
        g,
        &mut PrinterContext::default(),
        vec![Format::Svg.into()],
    )
    .unwrap();
}
```

### Structure

The structure pursues to follow the dot [notation](https://graphviz.org/doc/info/lang.html) closely, therefore it has
straight accordance. The structures can be found in `dot_structures::*` and has the following denotion:

```text
strict digraph t {                     : graph with t as id
        aa[color=green]                : node aa and attributes in []
        subgraph v {                   : subgraph v
         aa[shape=square]            : node aa in subgraph
         subgraph vv{a2 -> b2}       : another subgraph carrying edge inside( a type of the edge is Pair)
         aaa[color=red]
         aaa -> subgraph { d -> aaa} : subgraph id is anonymous id
        }
       aa -> be -> d -> aaa            : other edge with a type Chain
   }
```

### Generate a dot structure

The library provides a set of macros alleviating the process of graph construction.
The details including examples for every macros are given in the documentation for the macros 
and can be found in the [ `dot_generator::*`](dot-generator/src/lib.rs)  

#### Example

```rust
assert_eq!(
    node!("node_id"; attr!("atr1","val1"),attr!("atr2","val2")),
    node!(
        "node_id",
        vec![attr!("atr1", "val1"), attr!("atr2", "val2")]
    )
);

fn graph_test() {
    use dot_generator::*;
    use dot_structures::*;

    let g = r#"
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
            "#;

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
    );
}
```

### Attributes

The graphviz provides an enormous amount of possible [attributes](https://graphviz.org/doc/info/attrs.html) and to
support it, the library provides a set of structures alleviating the navigation among them namely:

- custom attribute can be easily compound with the macros `attr!(id,id)` nevertheless another possible formats:
- using named attributes like `graphviz_rust::attributes::color` for the `color` attribute
- using the particular
  structures `graphviz_rust::attributes::{EdgeAttributes,SubgraphAttributes GraphAttributes, NodeAttributes}`
  grouping and displaying which attribute belongs to the struct.

```rust
use dot_generator::*;
use dot_structures::*;
use graphviz_rust::attributes::{
    color, color_name, GraphAttributes, NodeAttributes,
};
use into_attr::IntoAttribute;

fn test() {
    assert_eq!(GraphAttributes::center(true), attr!("center", true));
    assert_eq!(
        NodeAttributes::color(color_name::antiquewhite1),
        attr!("color", "antiquewhite1")
    );
    assert_eq!(color::default().into_attr(), attr!("color", "black"));
}
```

### Transform into string following a dot format

The trait `DotPrinter` is summoned to transform a graph structure into string.

```rust
use dot_generator::*;
use dot_structures::*;
use graphviz_rust::printer::{DotPrinter, PrinterContext};

fn subgraph_test() {
    let mut ctx = PrinterContext::default();
    let s =
        subgraph!("id"; node!("abc"), edge!(node_id!("a") => node_id!("b")));

    assert_eq!(
        s.print(&mut ctx),
        "subgraph id {\n    abc\n    a -- b \n}".to_string()
    );
}
```

The module allows adjusting some parameters such as indent step or line separator using `PrinterContext`:

```rust
fn ctx() {
    use self::graphviz_rust::printer::PrinterContext;
    let mut ctx = PrinterContext::default();

    ctx.always_inline(); // everything in one line
    ctx.with_semi(); // semicolon at the end of every element
    ctx.with_indent_step(4); // indent 4 (default 2)
    ctx.with_inline_size(60); // size indicating the line needs to break into multilines
}
```

### External formats and others using cmd engine

The library provides an ability to use [command commands](https://graphviz.org/doc/info/command.html) from the rust
code.
The details are denoted in `graphviz_rust::{exec}` and `graphviz_rust::{exec_dot}` methods

```rust
fn output_test() {
    let mut g = graph!(id!("id"));
    exec(
        g,
        PrinterContext::default(),
        vec![
            Format::Svg.into(),
            CommandArg::Output("path_to_file".to_string()),
        ],
    );
}
```

### Caveats

#### The [command client](https://graphviz.org/download/) should be installed

Since, the library operates with a cmd client to execute the commands, the client should be installed beforehand, otherwise, the errors like: `No file or directory found` or `program not found` (depending on the OS) will be popped up.
