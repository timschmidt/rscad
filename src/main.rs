// data: images / ndarrays, points, lines, polylines, triangles / meshes, voxels / octrees
// functions: sum, difference, minkowski / gjk, wave function collapse
// ui: cli, editor / rendering window

#![deny(clippy::all)]
#![forbid(unsafe_code)]

use dxf::Drawing;
use dxf::entities::*;
use clap::{Arg, App};

fn main() {
    let matches = App::new("rscad")
        .version("0.1.0")
        .author("Timothy Schmidt <timschmidt@gmail.com>")
        .about("rscad is an interface to various CAD libraries written in Rust")
        .arg(Arg::with_name("input")
                 .short('i')
                 .long("input")
                 .takes_value(true)
                 .help("input CAD file"))
        .arg(Arg::with_name("output")
                 .short('o')
                 .long("output")
                 .takes_value(true)
                 .help("output CAD file"))
        .arg(Arg::with_name("transformation")
                 .short('t')
                 .long("transformation")
                 .takes_value(true)
                 .help("transformation to perform on CAD data"))
        .get_matches();

    let drawing_path = matches.value_of("input").unwrap_or("example.dxf");
    let drawing = Drawing::load_file(drawing_path).expect("loading file failed!");
    for e in drawing.entities() {
        println!("found entity on layer {}", e.common.layer);
        match e.specific {
            EntityType::Circle(ref circle) => {
                // do something with the circle
            },
            EntityType::Line(ref line) => {
                // do something with the line
            },
            _ => (),
        }
    }
}
