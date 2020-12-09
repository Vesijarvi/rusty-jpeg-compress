mod compress;
mod reader;
mod defines;

use compress::jpeg::*;
use reader::*;
use defines::*;

extern crate clap;
use clap::{App, Arg};
use std::fs;
use std::io::Read;
use crate::fs::File;


fn get_file_as_byte_vec(filename: &String)->Vec<u8> {
    let error_msg = "Error reading file: ".to_string() + filename;
    let mut f = File::open(&filename).expect(&error_msg);
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    
    buffer
}

fn main(){
    let matches = App::new("JPEG compress")
        .arg(
            Arg::with_name("compress")
                .short("c")
                .long("compress")
                .value_name("FILE")
                .takes_value(true),
        )
        .get_matches();
 
    if let Some(file) = matches.value_of("compress"){
        let byte_stream: Vec<u8> = get_file_as_byte_vec(&file.to_string());
        // println!("{:?}",byte_stream);

        compress(&byte_stream);

        // let output_file = file.to_string() + ".cmp";
        // let error_msg = "Error writing file: ".to_string();
    	// fs::write(output_file, compressed_data).expect(&error_msg);
    }
}
