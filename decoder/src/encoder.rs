![allow(non_snake_case)]
use crate::image::Image;

use std::io::BufReader;
use std::fs::File;

use std::f32::consts::PI;

use crate::primitives::*;
use crate::reader::data_reader;
use crate::image::Color;


pub fn encoder(reader: BufReader<File>) {
    // 1. 把 Image 切成 8x8 的 Block
    // 2. 對每一個 Block 做 DCT
    // 3. Reorder Zigzag
    // 4. Quantizize
    // 5. Encode Block to using huffman coding
    // 6. Add headers 
}