pub mod jpeg {

#![allow(non_snake_case)]

constant HEIGHT = 256;
constant WIDTH = 256;

#[derive(Clone, Copy)]
    pub struct Color {
        pub r: u8,
        pub g: u8,
        pub b: u8
    }

    impl Color {
        pub fn RGB(r: u8, g:u8, b:u8) -> Color {
            return Color {r, g, b};
        }
    }

    pub struct Image {
        pub width: u32,
        pub height: u32,
        pub pixels: Vec<Vec<Color>>
    }

    impl Image {
        // init to all black 
        pub fn new(width: u32, height:u32) -> Image {
            return Image {
                width: width,
                height: height,                                                               
                pixels: vec![vec![Color::RGB(0,0,0); width as usize]; height as usize]
            };
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
                    let myColor = Color::RGB(r,g,b);
                    color_vec.push(myColor);
                }
                _ => { panic!("Parse error in Color"); }
            }
            cnt = (cnt + 1) % 3;
        }

        color_vec
    }
}
