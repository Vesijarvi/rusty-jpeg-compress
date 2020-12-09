#![allow(non_snake_case)]
use crate::defines::*;


//     pub fn gray_to_block(stream_vec: &Vec<u8>) -> Vec<Vec<u8>> {
//         let mut image = Vec
//     }

// // for grayscale pictures
// pub fn read_to_mcus(stream_vec: &Vec<u8>) -> Vec<Vec<MCU>> {
//     // 256*256 image 
//     let w = 256 / 8;
//     let h = 256 / 8;
//     println!("Image with width: {}, height: {}.", w,h);

//     let mut _stream_vec = *stream_vec;
//     let mut MCUs = vec![vec!{Default::default(); w as usize}; h as usize];
//     for i in 0..h {
//         for j in 0..w {
//             MCUs[i as usize][j as usize] = stream_vec.pop().unwrap();
//         }
//     }
//     return MCUs;
// }


// Read steps
// [v] 1. Get in u8 string 
// [v] 2. Convert into YCbCr
// 3. Blockize 
