#![allow(non_snake_case)]

use crate define::*;

// // Transfer byte stream to Color 
//     pub fn vec8_to_color(stream_vec: &Vec<u8>) -> Vec<Color> {
//         let mut color_vec = Vec::new();

//         let mut r = 0;
//         let mut g = 0;
//         let mut b = 0;
//         let mut cnt = 0;

//         for c in stream_vec {
//             match cnt {
//                 0 => { r = *c; }
//                 1 => { g = *c; }
//                 2 => { 
//                     b = *c; 
//                     let myColor = Color::RGB(r,g,b);
//                     color_vec.push(myColor);
//                 }
//                 _ => { panic!("Parse error in Color"); }
//             }
//             cnt = (cnt + 1) % 3;
//         }

//         color_vec
//     }
//     pub fn gray_to_block(stream_vec: &Vec<u8>) -> Vec<Vec<u8>> {
//         let mut image = Vec
//     }

fn read_to_mcus(stream_vec: Vec<u8>) -> Vec<Vec<MCU>> {

}