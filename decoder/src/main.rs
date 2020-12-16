mod image; 
mod ppm;
mod decoder;
mod reader; 
mod marker;
mod primitives;

use image::Image;
use ppm::to_ppm;
use decoder::{decoder, show_mcu_stage};
use marker::marker_detector;
use reader::data_reader;

use std::fs::File;
use std::io::BufReader;

use std::str::FromStr;

extern crate clap;
use clap::{App, Arg, SubCommand};

fn main() -> std::io::Result<()> {

    // Usage Print

    let matches = App::new("JPEG Compression/Decompression")
        .arg(Arg::with_name("filename")
            .help("JPEG file path")
            .index(1)
            .required(true))
        .subcommand(SubCommand::with_name("marker")
            .about("only print marker"))
        .subcommand(SubCommand::with_name("ppm")
            .about("Output ppm format, filename: out.ppm"))
        .subcommand(SubCommand::with_name("mcu")
            .about("print different states of mcu")
            .arg(Arg::with_name("y")
                .help("Get mcu y coordinates from top to botton")
                .required(true)
                .index(1))
            .arg(Arg::with_name("x")
                .help("Get mcu y coordinates from left to right")
                .required(true)
                .index(2)))
        .subcommand(SubCommand::with_name("compress")
            .about("compress RAW to JPEG"))
        .get_matches();
            
    let filename = matches.value_of("filename").unwrap();
    println!("Filename: {}",filename);

    let f = File::open(filename)?;
    let reader = BufReader::new(f);

    match matches.subcommand_name() {
        Some("compress") => { 
            println!("COMPRESS"); 
            let image: Image = encoder(reader);
        }
        Some("marker") => { marker_detector(reader)?; }
        Some("reader") => { data_reader(reader); }
        Some("ppm") => { 
            let image: Image = decoder(reader);
            to_ppm(image)?;
        }
        Some("mcu") => {
            let y = matches.subcommand_matches("mcu").unwrap().value_of("y").unwrap();
            let x = matches.subcommand_matches("mcu").unwrap().value_of("x").unwrap();
            println!("{} {}", y, x);
            show_mcu_stage(reader, FromStr::from_str(y).unwrap(), FromStr::from_str(x).unwrap());
        }
        None => {
            let image: Image = decoder(reader);
            to_ppm(image)?;
        }
        Some(_) => { println!("unrechable"); } 
    }
    Ok(())
}
