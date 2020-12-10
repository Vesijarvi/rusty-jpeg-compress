#![allow(non_snake_case)]

// Read steps
// [v] 1. Get in u8 string 
// [v] 2. Convert into YCbCr
// [v] 3. Blockize 
// [ ] 4. DCT
// [ ] 5. quantilize
// [ ]
pub mod jpeg {
    use std::default::Default;

    pub const HEIGHT:usize = 256;
    pub const WIDTH:usize = 256;

    pub type Block = [[f32; 8]; 8];
    
    #[derive(Debug, Clone, Copy)]
    pub struct Color {
        pub Y: f32,
        pub Cb: f32,
        pub Cr: f32
    }
    
    impl Color {
        pub fn YCbCr(Y: f32, Cb:f32, Cr:f32) -> Color {
            return Color {Y, Cb, Cr};
        }
    }

    // zigzag block order
    const ZZ: [[usize; 8]; 8] = [
        [ 0,  1,  5,  6, 14, 15, 27, 28 ],
        [ 2,  4,  7, 13, 16, 26, 29, 42 ],
        [ 3,  8, 12, 17, 25, 30, 41, 43 ],
        [ 9, 11, 18, 24, 31, 40, 44, 53 ],
        [ 10, 19, 23, 32, 39, 45, 52, 54 ],
        [ 20, 22, 33, 38, 46, 51, 55, 60 ],
        [ 21, 34, 37, 47, 50, 56, 59, 61 ],
        [ 35, 36, 48, 49, 57, 58, 62, 63 ]
    ];
    // Transfer byte stream to Color 
    fn read_vec8_to_color(stream_vec: &Vec<u8>) -> Vec<Color> {
        let mut color_vec = Vec::new();
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        let mut cnt = 0;

        for c in stream_vec {
            match cnt {
                0 => { r = *c; }
                1 => { g = *c; }
                2 => { 
                    b = *c; 
                    // convert RGB to YCbCr
                    let y: f32 = 0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32;
                    let cb: f32 = -0.168 * r as f32  - 0.331 * g as f32 + 0.499 * b as f32;
                    let cr: f32 = 0.5 * r as f32 - 0.419 * g as f32 - 0.081 * b as f32;
                    let myColor = Color::YCbCr(y,cb,cr);
                    color_vec.push(myColor);
                }
                _ => { panic!("Parse error in Color"); }
            }
            cnt = (cnt + 1) % 3;
        }
        color_vec
    }

    fn read_color_to_mcus(color_vec: &Vec<Color>) -> Vec<Vec<Block>>  {
        // still can make improvement to simplify code
        // lets just leave it for now
        let mut Y_plane = Vec::new();
        let mut Cb_plane = Vec::new();
        let mut Cr_plane = Vec::new();

        for f in color_vec {
            let y:f32 = f.Y;
            let cb:f32 = f.Cb;
            let cr:f32 = f.Cr; 
            Y_plane.push(y);
            Cb_plane.push(cb);
            Cr_plane.push(cr);
        }
        let mut bitplanes: Vec<Vec<Block>> = vec![vec![Default::default(); 64]; 3];
    
        for y in 0..256 {
            for x in 0..256 {
                bitplanes[0][(x / 8) + 4 * (y / 8)][x % 8][y % 8] = Y_plane[x + 256 * y];
            }
        }
        for y in 0..256 {
            for x in 0..256 {
                bitplanes[1][(x / 8) + 4 * (y / 8)][x % 8][y % 8] = Cb_plane[x + 256 * y];
            }
        }
        for y in 0..256 {
            for x in 0..256 {
                bitplanes[2][(x / 8) + 4 * (y / 8)][x % 8][y % 8] = Cr_plane[x + 256 * y];
            }
        }
        bitplanes
    }
    /*
    fn dct(in_mcus: &Vec<Vec<Block>>) -> Vec<Vec<Block>> {

    }*/
    pub fn compress(stream_vec: &Vec<u8>) {
        let colorYCbCr = read_vec8_to_color(&stream_vec);
        // println!("{:?}",myColor);
        let my_mcus = read_color_to_mcus(&colorYCbCr);
        
    }
}
