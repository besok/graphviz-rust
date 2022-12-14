//! It allows to execute cmd engine passing extra parameters
//!
//! *It is important*: to execute it properly it needs to have an [`executable package`] on the system
//!
//! The extra information can be found in [`layouts`] and [`outputs`]
//!
//! [`layouts`]: https://graphviz.org/docs/layouts/
//! [`outputs`]:https://graphviz.org/docs/outputs/
//! [`executable package`]: https://graphviz.org/download/
//! # Example:
//! ```no_run
//!     use dot_structures::*;
//!     use dot_generator::*;
//!     use graphviz_rust::attributes::*;
//!     use graphviz_rust::cmd::{CommandArg, Format};
//!     use graphviz_rust::exec;
//!     use graphviz_rust::printer::{PrinterContext,DotPrinter};
//!
//!  fn graph_to_output(){
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
//!        let graph_svg = exec(g, &mut PrinterContext::default(), vec![
//!            CommandArg::Format(Format::Svg),
//!        ]).unwrap();
//!
//!  }
//!  fn graph_to_file(){
//!     let mut g = graph!(id!("id"));
//!         let mut ctx = PrinterContext::default();
//!         ctx.always_inline();
//!         let empty = exec(g, &mut ctx, vec![
//!            CommandArg::Format(Format::Svg),
//!            CommandArg::Output("1.svg".to_string())
//!        ]).unwrap();
//!
//!  }
//! ```
use std::{
    io::{self, Write},
    process::{Command, Output},
};

use tempfile::NamedTempFile;

pub(crate) fn exec(graph: String, args: Vec<CommandArg>) -> io::Result<String> {
    let args = args.into_iter().map(|a| a.prepare()).collect();
    temp_file(graph).and_then(|f| {
        let path = f.path().to_string_lossy().to_string();
        do_exec(path, args).map(|o| {
            if o.status.code().map(|c| c != 0).unwrap_or(true) {
                String::from_utf8_lossy(&*o.stderr).to_string()
            } else {
                String::from_utf8_lossy(&*o.stdout).to_string()
            }
        })
    })
}

fn do_exec(input: String, args: Vec<String>) -> io::Result<Output> {
    let mut command = Command::new("dot");

    for arg in args {
        command.arg(arg);
    }
    command.arg(input).output()
}

fn temp_file(ctx: String) -> io::Result<NamedTempFile> {
    let mut file = NamedTempFile::new()?;
    file.write_all(ctx.as_bytes()).map(|_x| file)
}

/// Command arguments that can be passed to exec.
/// The list of possible [`commands`]
///
/// [`commands`]:https://graphviz.org/doc/info/command.html
pub enum CommandArg {
    /// any custom argument.
    ///
    /// _Note_: it does not manage any prefixes and thus '-' or the prefix must be passed as well.
    Custom(String),
    /// Regulates the output file with -o prefix
    Output(String),
    /// [`Layouts`] in cmd
    ///
    /// [`Layouts`]: https://graphviz.org/docs/layouts/
    Layout(Layout),
    /// [`Output`] formats in cmd
    ///
    /// [`Output`]:https://graphviz.org/docs/outputs/
    Format(Format),
}

impl CommandArg {
    fn prepare(&self) -> String {
        match self {
            CommandArg::Custom(s) => s.clone(),
            CommandArg::Output(p) => format!("-o{}", p),
            CommandArg::Layout(l) => format!("-K{}", format!("{:?}", l).to_lowercase()),
            CommandArg::Format(f) => {
                let str = match f {
                    Format::Xdot12 => "xdot1.2".to_string(),
                    Format::Xdot14 => "xdot1.4".to_string(),
                    Format::ImapNp => "imap_np".to_string(),
                    Format::CmapxNp => "cmapx_np".to_string(),
                    Format::DotJson => "dot_json".to_string(),
                    Format::XdotJson => "xdot_json".to_string(),
                    Format::PlainExt => "plain-ext".to_string(),
                    _ => format!("{:?}", f).to_lowercase(),
                };
                format!("-T{}", str)
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Layout {
    Dot,
    Neato,
    Twopi,
    Circo,
    Fdp,
    Asage,
    Patchwork,
    Sfdp,
}

#[derive(Debug, Copy, Clone)]
pub enum Format {
    Bmp,
    Cgimage,
    Canon,
    Dot,
    Gv,
    Xdot,
    Xdot12,
    Xdot14,
    Eps,
    Exr,
    Fig,
    Gd,
    Gd2,
    Gif,
    Gtk,
    Ico,
    Cmap,
    Ismap,
    Imap,
    Cmapx,
    ImapNp,
    CmapxNp,
    Jpg,
    Jpeg,
    Jpe,
    Jp2,
    Json,
    Json0,
    DotJson,
    XdotJson,
    Pdf,
    Pic,
    Pct,
    Pict,
    Plain,
    PlainExt,
    Png,
    Pov,
    Ps,
    Ps2,
    Psd,
    Sgi,
    Svg,
    Svgz,
    Tga,
    Tif,
    Tiff,
    Tk,
    Vml,
    Vmlz,
    Vrml,
    Vbmp,
    Webp,
    Xlib,
    X11,
}
