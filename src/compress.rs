#![allow(non_snake_case)]

// Read steps
// [v] 1. Get in u8 string 
// [v] 2. Convert into YCbCr
// [ ] 3. Blockize 
pub mod jpeg {
    
    pub const HEIGHT:usize = 256;
    pub const WIDTH:usize = 256;
    
    pub type Block = [[f32; 8]; 8];
    
    /* Minimum Coded Unit */
    pub type MCU = [Vec<Vec<Block>>; 3];

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
    
    // Transfer byte stream to Color 
    fn vec8_to_color(stream_vec: &Vec<u8>) -> Vec<Color> {
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
                    let y:f32 = 0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32;
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
    // into three plane
    fn to_three_plane(color_vec: Vec<Color>) {
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


    }
    pub fn compress(stream_vec: &Vec<u8>) {
        let myColor = vec8_to_color(&stream_vec);
        // println!("{:?}",myColor);
        
    }
}
