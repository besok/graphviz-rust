//! graphviz [`attributes`]
//!
//! [`attributes`]: https://graphviz.org/doc/info/attrs.html
//! # Examples:
//! ```rust
//! use graphviz_rust::attributes::{color, color_name, GraphAttributes, NodeAttributes};
//! use into_attr::IntoAttribute;
//! use dot_structures::*;
//! use dot_generator::*;
//! fn test() {
//!         assert_eq!(GraphAttributes::center(true), attr!("center",true));
//!         assert_eq!(
//!             NodeAttributes::color(color_name::antiquewhite1),
//!             attr!("color","antiquewhite1"));
//!         assert_eq!(color::default().into_attr(), attr!("color","black"));
//!     }
//! ```
mod generate;
use crate::{generate_attr, as_item};
use into_attr::IntoAttribute;
use into_attr_derive::IntoAttribute;
use dot_generator::{attr, id};
use dot_structures::*;
use std::fmt::Display;
use std::fmt::Formatter;

/// The attributes appearing on the node
pub enum  NodeAttributes {}
/// The attributes appearing on the edge
pub enum  EdgeAttributes {}
/// The attributes appearing on the root graph
pub enum  GraphAttributes {}
/// The attributes appearing on the subgraph
pub enum  SubgraphAttributes {}

generate_attr!(struct _background for GraphAttributes; String; "<none>".to_string() );
generate_attr!(struct area for NodeAttributes, SubgraphAttributes; f32; 1.0);
generate_attr!(enum arrowhead for EdgeAttributes;
    //region values
    normal,dot,odot,none,empty,diamond,ediamond,odiamond,box_,open,vee,halfopen,obox,
    crow,invempty,tee,invodot,invdot,inv;
    normal
    //endregion
);
generate_attr!(enum arrowtail for EdgeAttributes;
    //region values
    normal,dot,odot,none,empty,diamond,ediamond,odiamond,box_,open,vee,halfopen,obox,
    crow,invempty,tee,invodot,invdot,inv;
    normal
    //endregion
);
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
    //region values
    box_,polygon,ellipse,oval,circle,point,egg,triangle,plaintext,plain,diamond,trapezium,
    parallelogram,house,pentagon,hexagon,septagon,octagon,doublecircle,doubleoctagon,tripleoctagon,
    invtriangle,invtrapezium,invhouse,Mdiamond,Msquare,Mcircle,rect,rectangle,square,star,none,
    underline,cylinder,note,tab,folder,box3d,component,promoter,cds,terminator,utr,primersite,
    restrictionsite,fivepoverhang,threepoverhang,noverhang,assembly,signature,insulator,ribosite,
    rnastab,proteasesite,proteinstab,rpromoter,rarrow,larrow,lpromoter;
    ellipse
    //endregion
);
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
generate_attr!(struct bgcolor for GraphAttributes,SubgraphAttributes; color_name);
generate_attr!(struct color for EdgeAttributes,SubgraphAttributes,NodeAttributes; color_name;color_name::black);
generate_attr!(struct fillcolor for EdgeAttributes,SubgraphAttributes,NodeAttributes; color_name);
generate_attr!(struct fontcolor for GraphAttributes,EdgeAttributes,SubgraphAttributes,NodeAttributes; color_name);
generate_attr!(struct labelfontcolor for EdgeAttributes; color_name;color_name::black);
generate_attr!(struct pencolor for SubgraphAttributes; color_name;color_name::black);

// support of the x11 color scheme
generate_attr!(enum color_name;
    //region values
    aliceblue,antiquewhite,antiquewhite1,antiquewhite2,antiquewhite3,antiquewhite4,aqua,aquamarine,
    aquamarine1,aquamarine2,aquamarine3,aquamarine4,azure,azure1,azure2,azure3,azure4,beige,bisque,
    bisque1,bisque2,bisque3,bisque4,black,blanchedalmond,blue,blue1,blue2,blue3,blue4,blueviolet,
    brown,brown1,brown2,brown3,brown4,burlywood,burlywood1,burlywood2,burlywood3,burlywood4,
    cadetblue,cadetblue1,cadetblue2,cadetblue3,cadetblue4,chartreuse,chartreuse1,chartreuse2,
    chartreuse3,chartreuse4,chocolate,chocolate1,chocolate2,chocolate3,chocolate4,coral,coral1,
    coral2,coral3,coral4,cornflowerblue,cornsilk,cornsilk1,cornsilk2,cornsilk3,cornsilk4,crimson,
    cyan,cyan1,cyan2,cyan3,cyan4,darkblue,darkcyan,darkgoldenrod,darkgoldenrod1,darkgoldenrod2,
    darkgoldenrod3,darkgoldenrod4,darkgray,darkgreen,darkgrey,darkkhaki,darkmagenta,darkolivegreen,
    darkolivegreen1,darkolivegreen2,darkolivegreen3,darkolivegreen4,darkorange,darkorange1,
    darkorange2,darkorange3,darkorange4,darkorchid,darkorchid1,darkorchid2,darkorchid3,darkorchid4,
    darkred,darksalmon,darkseagreen,darkseagreen1,darkseagreen2,darkseagreen3,darkseagreen4,
    darkslateblue,darkslategray,darkslategray1,darkslategray2,darkslategray3,darkslategray4,
    darkslategrey,darkturquoise,darkviolet,deeppink,deeppink1,deeppink2,deeppink3,deeppink4,
    deepskyblue,deepskyblue1,deepskyblue2,deepskyblue3,deepskyblue4,dimgray,dimgrey,dodgerblue,
    dodgerblue1,dodgerblue2,dodgerblue3,dodgerblue4,firebrick,firebrick1,firebrick2,firebrick3,
    firebrick4,floralwhite,forestgreen,fuchsia,gainsboro,ghostwhite,gold,gold1,gold2,gold3,
    gold4,goldenrod,goldenrod1,goldenrod2,goldenrod3,goldenrod4,gray,gray0,gray1,gray10,
    gray100,gray11,gray12,gray13,gray14,gray15,gray16,gray17,gray18,gray19,gray2,gray20,gray21,
    gray22,gray23,gray24,gray25,gray26,gray27,gray28,gray29,gray3,gray30,gray31,gray32,gray33,
    gray34,gray35,gray36,gray37,gray38,gray39,gray4,gray40,gray41,gray42,gray43,gray44,gray45,
    gray46,gray47,gray48,gray49,gray5,gray50,gray51,gray52,gray53,gray54,gray55,gray56,gray57,
    gray58,gray59,gray6,gray60,gray61,gray62,gray63,gray64,gray65,gray66,gray67,gray68,gray69,
    gray7,gray70,gray71,gray72,gray73,gray74,gray75,gray76,gray77,gray78,gray79,gray8,gray80,
    gray81,gray82,gray83,gray84,gray85,gray86,gray87,gray88,gray89,gray9,gray90,gray91,gray92,
    gray93,gray94,gray95,gray96,gray97,gray98,gray99,green,green1,green2,green3,green4,greenyellow,
    grey,grey0,grey1,grey10,grey100,grey11,grey12,grey13,grey14,grey15,grey16,grey17,grey18,
    grey19,grey2,grey20,grey21,grey22,grey23,grey24,grey25,grey26,grey27,grey28,grey29,grey3,
    grey30,grey31,grey32,grey33,grey34,grey35,grey36,grey37,grey38,grey39,grey4,grey40,grey41,
    grey42,grey43,grey44,grey45,grey46,grey47,grey48,grey49,grey5,grey50,grey51,grey52,grey53,
    grey54,grey55,grey56,grey57,grey58,grey59,grey6,grey60,grey61,grey62,grey63,grey64,grey65,
    grey66,grey67,grey68,grey69,grey7,grey70,grey71,grey72,grey73,grey74,grey75,grey76,grey77,
    grey78,grey79,grey8,grey80,grey81,grey82,grey83,grey84,grey85,grey86,grey87,grey88,grey89,
    grey9,grey90,grey91,grey92,grey93,grey94,grey95,grey96,grey97,grey98,grey99,honeydew,honeydew1,
    honeydew2,honeydew3,honeydew4,hotpink,hotpink1,hotpink2,hotpink3,hotpink4,indianred,indianred1,
    indianred2,indianred3,indianred4,indigo,invis,ivory,ivory1,ivory2,ivory3,ivory4,khaki,khaki1,
    khaki2,khaki3,khaki4,lavender,lavenderblush,lavenderblush1,lavenderblush2,lavenderblush3,
    lavenderblush4,lawngreen,lemonchiffon,lemonchiffon1,lemonchiffon2,lemonchiffon3,lemonchiffon4,
    lightblue,lightblue1,lightblue2,lightblue3,lightblue4,lightcoral,lightcyan,lightcyan1,
    lightcyan2,lightcyan3,lightcyan4,lightgoldenrod,lightgoldenrod1,lightgoldenrod2,lightgoldenrod3,
    lightgoldenrod4,lightgoldenrodyellow,lightgray,lightgreen,lightgrey,lightpink,lightpink1,
    lightpink2,lightpink3,lightpink4,lightsalmon,lightsalmon1,lightsalmon2,lightsalmon3,
    lightsalmon4,lightseagreen,lightskyblue,lightskyblue1,lightskyblue2,lightskyblue3,
    lightskyblue4,lightslateblue,lightslategray,lightslategrey,lightsteelblue,lightsteelblue1,
    lightsteelblue2,lightsteelblue3,lightsteelblue4,lightyellow,lightyellow1,lightyellow2,
    lightyellow3,lightyellow4,lime,limegreen,linen,magenta,magenta1,magenta2,magenta3,
    magenta4,maroon,maroon1,maroon2,maroon3,maroon4,mediumaquamarine,mediumblue,mediumorchid,
    mediumorchid1,mediumorchid2,mediumorchid3,mediumorchid4,mediumpurple,mediumpurple1,
    mediumpurple2,mediumpurple3,mediumpurple4,mediumseagreen,mediumslateblue,mediumspringgreen,
    mediumturquoise,mediumvioletred,midnightblue,mintcream,mistyrose,mistyrose1,mistyrose2,
    mistyrose3,mistyrose4,moccasin,navajowhite,navajowhite1,navajowhite2,navajowhite3,navajowhite4,
    navy,navyblue,none,oldlace,olive,olivedrab,olivedrab1,olivedrab2,olivedrab3,olivedrab4,
    orange,orange1,orange2,orange3,orange4,orangered,orangered1,orangered2,orangered3,orangered4,
    orchid,orchid1,orchid2,orchid3,orchid4,palegoldenrod,palegreen,palegreen1,palegreen2,
    palegreen3,palegreen4,paleturquoise,paleturquoise1,paleturquoise2,paleturquoise3,paleturquoise4,
    palevioletred,palevioletred1,palevioletred2,palevioletred3,palevioletred4,papayawhip,
    peachpuff,peachpuff1,peachpuff2,peachpuff3,peachpuff4,peru,pink,pink1,pink2,pink3,pink4,
    plum,plum1,plum2,plum3,plum4,powderblue,purple,purple1,purple2,purple3,purple4,rebeccapurple,
    red,red1,red2,red3,red4,rosybrown,rosybrown1,rosybrown2,rosybrown3,rosybrown4,royalblue,
    royalblue1,royalblue2,royalblue3,royalblue4,saddlebrown,salmon,salmon1,salmon2,salmon3,salmon4,
    sandybrown,seagreen,seagreen1,seagreen2,seagreen3,seagreen4,seashell,seashell1,seashell2,
    seashell3,seashell4,sienna,sienna1,sienna2,sienna3,sienna4,silver,skyblue,skyblue1,skyblue2,
    skyblue3,skyblue4,slateblue,slateblue1,slateblue2,slateblue3,slateblue4,slategray,slategray1,
    slategray2,slategray3,slategray4,slategrey,snow,snow1,snow2,snow3,snow4,springgreen,
    springgreen1,springgreen2,springgreen3,springgreen4,steelblue,steelblue1,steelblue2,steelblue3,
    steelblue4,tan,tan1,tan2,tan3,tan4,teal,thistle,thistle1,thistle2,thistle3,thistle4,tomato,
    tomato1,tomato2,tomato3,tomato4,transparent,turquoise,turquoise1,turquoise2,turquoise3,
    turquoise4,violet,violetred,violetred1,violetred2,violetred3,violetred4,webgray,webgreen,
    webgrey,webmaroon,webpurple,wheat,wheat1,wheat2,wheat3,wheat4,white,whitesmoke,x11gray,
    x11green,x11grey,x11maroon,x11purple,yellow,yellow1,yellow2,yellow3,yellow4,yellowgreen
    //endregion
);





#[cfg(test)]
pub mod tests {
    use std::fmt::{Display, Formatter};
    use dot_generator::{attr};
    use crate::attributes::*;
    use into_attr::IntoAttribute;


    #[test]
    fn test() {
        assert_eq!(GraphAttributes::center(true), attr!("center",true));
        assert_eq!(GraphAttributes::bgcolor(color_name::antiquewhite1), attr!("bgcolor","antiquewhite1"));
        assert_eq!(color::default().into_attr(), attr!("color","black"));
    }
}