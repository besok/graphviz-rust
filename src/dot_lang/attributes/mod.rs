//! graphviz [`attributes`]
//!
//! [`attributes`]: https://graphviz.org/doc/info/attrs.html
mod generate;

use crate::{generate_attr, as_item};
use into_attr::IntoAttribute;
use into_attr_derive::IntoAttribute;
use dot_generator::{attr, id};
use dot_structures::*;

struct NodeAttributes {}

struct EdgeAttributes {}

struct GraphAttributes {}

struct SubgraphAttributes {}

generate_attr!(struct _background for GraphAttributes; String; "<none>".to_string() );
generate_attr!(struct area for NodeAttributes, SubgraphAttributes; f32; 1.0);
generate_attr!(enum arrowhead for EdgeAttributes;
    normal,dot,odot,none,empty,diamond,ediamond,odiamond,box_,open,vee,halfopen,obox,
    crow,invempty,tee,invodot,invdot,inv;
    normal);
generate_attr!(enum arrowtail for EdgeAttributes;
    normal,dot,odot,none,empty,diamond,ediamond,odiamond,box_,open,vee,halfopen,obox,
    crow,invempty,tee,invodot,invdot,inv;
    normal);
generate_attr!(struct arrowsize for EdgeAttributes; f32; 0.0);
generate_attr!(struct bb for GraphAttributes; String);
generate_attr!(struct center for GraphAttributes; bool;false);
generate_attr!(struct charset for GraphAttributes; String;"UTF-8".to_string());
generate_attr!(struct class for GraphAttributes,NodeAttributes,EdgeAttributes,SubgraphAttributes;String;"".to_string());
generate_attr!(struct colorscheme for GraphAttributes,NodeAttributes,EdgeAttributes,SubgraphAttributes;String;"".to_string());
generate_attr!(struct comment for GraphAttributes,NodeAttributes,EdgeAttributes;String;"".to_string());
generate_attr!(struct compound for GraphAttributes;bool;false);
generate_attr!(struct concentrate for GraphAttributes;bool;false);
generate_attr!(struct Damping for GraphAttributes;f32;0.99);
generate_attr!(struct decorate for EdgeAttributes;bool;false);
generate_attr!(struct defaultdist for GraphAttributes;f32);
generate_attr!(struct constraint for EdgeAttributes;bool;true);
generate_attr!(struct dim for GraphAttributes;i32;2);
generate_attr!(struct dimen for GraphAttributes;i32;2);
generate_attr!(enum clusterrank for GraphAttributes; local,global,none; local);
generate_attr!(enum dir for EdgeAttributes; forward,back,both,none; forward);
generate_attr!(struct diredgeconstraints for GraphAttributes; bool; false);
generate_attr!(struct distortion for NodeAttributes; f32; 0.0);
generate_attr!(struct dpi for GraphAttributes; f32; 96.0);
generate_attr!(struct dedgehrefpi for EdgeAttributes; String);
generate_attr!(struct edgetarget for EdgeAttributes; String );
generate_attr!(struct edgetooltip for EdgeAttributes; String );
generate_attr!(struct edgeURL for EdgeAttributes; String );
generate_attr!(struct epsilon for GraphAttributes; f32);
generate_attr!(struct esep for GraphAttributes; f32;3.0);
generate_attr!(struct fixedsize for NodeAttributes; bool;false);
generate_attr!(struct fontname for NodeAttributes,GraphAttributes,EdgeAttributes,SubgraphAttributes; String;"Times-Roman".to_string());
generate_attr!(struct href for NodeAttributes,GraphAttributes,EdgeAttributes,SubgraphAttributes; String;"".to_string());
generate_attr!(struct id for NodeAttributes,GraphAttributes,EdgeAttributes,SubgraphAttributes; String;"".to_string());
generate_attr!(struct fontsize for NodeAttributes,GraphAttributes,EdgeAttributes,SubgraphAttributes; f32;14.0);
generate_attr!(struct fontnames for GraphAttributes; String;"".to_string());
generate_attr!(struct fontpath for GraphAttributes; String;"<system-dependent>".to_string());
generate_attr!(struct forcelabels for GraphAttributes; bool;true);
generate_attr!(struct gradientangle for NodeAttributes,GraphAttributes,SubgraphAttributes; i32);
generate_attr!(struct group for NodeAttributes; String);
generate_attr!(struct head_lp for EdgeAttributes; String);
generate_attr!(struct headclip for EdgeAttributes; bool;true);
generate_attr!(struct headhref for EdgeAttributes; String;"".to_string());
generate_attr!(struct headlabel for EdgeAttributes; String;"".to_string());
generate_attr!(struct headtarget for EdgeAttributes; String;"".to_string());
generate_attr!(struct headtooltip for EdgeAttributes; String;"".to_string());
generate_attr!(struct headURL for EdgeAttributes; String;"".to_string());
generate_attr!(struct height for NodeAttributes; f32;0.5);
generate_attr!(struct image for NodeAttributes; String;"".to_string());
generate_attr!(struct imagepos for NodeAttributes; String;"mc".to_string());
generate_attr!(struct imagescale for NodeAttributes; bool;false);
generate_attr!(struct imagepath for GraphAttributes; String;"".to_string());
generate_attr!(struct inputscale for GraphAttributes; f32);
generate_attr!(struct K for GraphAttributes,SubgraphAttributes; f32;0.3);
generate_attr!(struct label for NodeAttributes,GraphAttributes,EdgeAttributes,SubgraphAttributes; String;"".to_string());
generate_attr!(struct label_scheme for  GraphAttributes; i32;0);
generate_attr!(struct labelangle for  EdgeAttributes; f32;-25.0);
generate_attr!(struct labeldistance for  EdgeAttributes; f32;1.0);
generate_attr!(struct labelfloat for  EdgeAttributes; bool;false);
generate_attr!(struct labelfontname for  EdgeAttributes; String;"Times-Roman".to_string());
generate_attr!(struct labelfontsize for  EdgeAttributes; f32;14.0);
generate_attr!(struct labelhref for  EdgeAttributes; String;"".to_string());
generate_attr!(struct labeljust for  GraphAttributes,SubgraphAttributes; String;"c".to_string());
generate_attr!(struct labelloc for  GraphAttributes,SubgraphAttributes,NodeAttributes; String);
generate_attr!(struct labeltarget for EdgeAttributes; String);
generate_attr!(struct labeltooltip for EdgeAttributes; String);
generate_attr!(struct labelURL for EdgeAttributes; String);
generate_attr!(struct landscape for GraphAttributes; bool;false);
generate_attr!(struct layer for EdgeAttributes,NodeAttributes,SubgraphAttributes; String);
generate_attr!(struct layerlistsep for GraphAttributes; String;",".to_string());
generate_attr!(struct layers for GraphAttributes; String);
generate_attr!(struct layerselect for GraphAttributes; String);
generate_attr!(struct layersep for GraphAttributes; String;":\t ".to_string());
generate_attr!(struct layout for GraphAttributes; String);
generate_attr!(struct len for EdgeAttributes; f32);
generate_attr!(struct levels for GraphAttributes; i32);
generate_attr!(struct levelsgap for GraphAttributes; f32;0.0);
generate_attr!(struct lhead for EdgeAttributes; String);
generate_attr!(struct lheight for GraphAttributes,SubgraphAttributes; f32);
generate_attr!(struct lwidth for GraphAttributes,SubgraphAttributes; f32);
generate_attr!(struct lp for GraphAttributes,SubgraphAttributes,EdgeAttributes; String);
generate_attr!(struct margin for GraphAttributes,SubgraphAttributes,NodeAttributes; f32);
generate_attr!(struct ltail for EdgeAttributes; String);
generate_attr!(struct maxiter for GraphAttributes; i32);
generate_attr!(struct mclimit for GraphAttributes; f32;1.);
generate_attr!(struct mindist for GraphAttributes; f32;1.);
generate_attr!(struct minlen for EdgeAttributes; i32;1);
generate_attr!(struct mode for GraphAttributes; String;"major".to_string());
generate_attr!(struct model for GraphAttributes; String;"shortpath".to_string());
generate_attr!(struct mosek for GraphAttributes; bool;false);
generate_attr!(struct newrank for GraphAttributes; bool;false);
generate_attr!(struct nodesep for GraphAttributes; f32;0.25);
generate_attr!(struct normalize for GraphAttributes; bool;false);
generate_attr!(struct notranslate for GraphAttributes; bool;false);
generate_attr!(struct nslimit for GraphAttributes; f32);
generate_attr!(struct nslimit1 for GraphAttributes; f32);
generate_attr!(struct ordering for GraphAttributes,NodeAttributes; String);
generate_attr!(struct orientation for GraphAttributes,NodeAttributes; f32;0.);
generate_attr!(enum outputorder for GraphAttributes; breadthfirst,nodesfirst,edgesfirst;breadthfirst);
generate_attr!(struct nojustify for GraphAttributes,NodeAttributes,SubgraphAttributes,EdgeAttributes; bool;false);
generate_attr!(struct overlap for GraphAttributes; bool;true);
generate_attr!(struct overlap_shrink for GraphAttributes; bool;true);
generate_attr!(struct pack for GraphAttributes; bool;false);
generate_attr!(struct overlap_scaling for GraphAttributes; f32;-4.);
generate_attr!(struct pad for GraphAttributes; f32;0.0555);
generate_attr!(struct page for GraphAttributes; f32);
generate_attr!(enum packmode for GraphAttributes; node,clust,graph,array;node);
generate_attr!(enum pagedir for GraphAttributes; BL,BR,TL,TR,RB,RT,LB,LT;BL);
generate_attr!(struct penwidth for SubgraphAttributes,NodeAttributes,EdgeAttributes; f32;1.);
generate_attr!(struct peripheries for SubgraphAttributes,NodeAttributes; i32);
generate_attr!(struct pin for NodeAttributes; bool;false);
generate_attr!(struct pos for NodeAttributes,EdgeAttributes; String);
generate_attr!(enum quadtree for GraphAttributes; normal,fast,none;normal);
generate_attr!(struct quantum for GraphAttributes; f32;0.);
generate_attr!(enum rank for SubgraphAttributes; same,min,source,max,sink);
generate_attr!(enum rankdir for GraphAttributes; TB,BT,LR,RL;TB);
generate_attr!(struct ranksep for GraphAttributes; f32;0.5);
generate_attr!(struct ratio for GraphAttributes; f32);
generate_attr!(struct rects for NodeAttributes; String);
generate_attr!(struct regular for NodeAttributes; bool;false);
generate_attr!(struct remincross for GraphAttributes; bool;true);
generate_attr!(struct repulsiveforce for GraphAttributes; f32;1.);
generate_attr!(struct resolution for GraphAttributes; f32;96.);
generate_attr!(struct root for GraphAttributes,NodeAttributes; String);
generate_attr!(struct rotate for GraphAttributes; i32;0);
generate_attr!(struct rotation for GraphAttributes; f32;0.);
generate_attr!(struct samehead for EdgeAttributes; String);
generate_attr!(struct sametail for EdgeAttributes; String);
generate_attr!(struct samplepoints for NodeAttributes; i32;8);
generate_attr!(struct searchsize for GraphAttributes; i32;30);
generate_attr!(struct scale for GraphAttributes; f32);
generate_attr!(struct sep for GraphAttributes; f32;4.);
generate_attr!(struct shapefile for NodeAttributes; String);
generate_attr!(enum shape for NodeAttributes;
    box_,polygon,ellipse,oval,circle,point,egg,triangle,plaintext,plain,diamond,trapezium,
    parallelogram,house,pentagon,hexagon,septagon,octagon,doublecircle,doubleoctagon,tripleoctagon,
    invtriangle,invtrapezium,invhouse,Mdiamond,Msquare,Mcircle,rect,rectangle,square,star,none,
    underline,cylinder,note,tab,folder,box3d,component,promoter,cds,terminator,utr,primersite,
    restrictionsite,fivepoverhang,threepoverhang,noverhang,assembly,signature,insulator,ribosite,
    rnastab,proteasesite,proteinstab,rpromoter,rarrow,larrow,lpromoter
    ;
    ellipse);
generate_attr!(struct showboxes for NodeAttributes,GraphAttributes,EdgeAttributes; i32;0);
generate_attr!(struct sides for NodeAttributes; i32;4);
generate_attr!(struct skew for NodeAttributes; f32;0.);
generate_attr!(enum smoothing for GraphAttributes; none,avg_dist,graph_dist,power_dist,rng,spring,triangle;none);
generate_attr!(struct size for GraphAttributes; f32);
generate_attr!(struct sortv for GraphAttributes,SubgraphAttributes,NodeAttributes; i32;0);
generate_attr!(struct splines for GraphAttributes; bool);
generate_attr!(struct start for GraphAttributes; String);
generate_attr!(struct style for GraphAttributes,EdgeAttributes,NodeAttributes,SubgraphAttributes; String);
generate_attr!(struct stylesheet for GraphAttributes; String);
generate_attr!(struct tail_lp for EdgeAttributes; String);
generate_attr!(struct tailhref for EdgeAttributes; String);
generate_attr!(struct taillabel for EdgeAttributes; String);
generate_attr!(struct tailtarget for EdgeAttributes; String);
generate_attr!(struct tailtooltip for EdgeAttributes; String);
generate_attr!(struct tailURL for EdgeAttributes; String);
generate_attr!(struct target for EdgeAttributes,GraphAttributes,NodeAttributes,SubgraphAttributes; String);
generate_attr!(struct tooltip for EdgeAttributes,GraphAttributes,NodeAttributes,SubgraphAttributes; String);
generate_attr!(struct URL for EdgeAttributes,GraphAttributes,NodeAttributes,SubgraphAttributes; String);
generate_attr!(struct tailclip for EdgeAttributes; bool;true);
generate_attr!(struct truecolor for GraphAttributes; bool);
generate_attr!(struct vertices for NodeAttributes; String);
generate_attr!(struct viewport for GraphAttributes; String);
generate_attr!(struct voro_margin for GraphAttributes; f32;0.05);
generate_attr!(struct weight for EdgeAttributes; i32;1);
generate_attr!(struct width for NodeAttributes; f32;0.75);
generate_attr!(struct xdotversion for GraphAttributes; String);
generate_attr!(struct xlabel for EdgeAttributes,NodeAttributes; String);
generate_attr!(struct xlp for EdgeAttributes,NodeAttributes; String);
generate_attr!(struct z for NodeAttributes; f32;0.);



// bgcolor / color / fillcolor / fontcolor / labelfontcolor /pencolor

#[cfg(test)]
pub mod tests {
    use crate::dot_lang::attributes::*;
    use dot_generator::{attr};
    use crate::dot_lang::attributes::*;
    use into_attr::IntoAttribute;

    #[test]
    fn shape_test() {
        assert_eq!(GraphAttributes::center(true), attr!("center",true));
    }
}