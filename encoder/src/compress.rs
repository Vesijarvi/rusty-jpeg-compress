#![allow(non_snake_case)]

pub mod jpeg {
    use std::default::Default;
    use std::f32::consts::PI;

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

    fn cc(i: usize, j: usize) -> f32 {
        if i == 0 && j == 0 {
            return 1.0/2.0;
        } else if i == 0 || j == 0 {
            return 1.0 / (2.0 as f32).sqrt();
        } else {
            return 1.0;
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
    // quantize table
    const QTable: [[usize; 8]; 8] = [
        [ 16, 11, 10, 16, 24, 40, 51, 61],
        [ 12, 12, 14, 19, 26, 58, 60, 55],
        [ 14, 13, 16, 24, 40, 57, 69, 56],
        [ 14, 17, 22, 29, 51, 87, 80, 62],
        [ 18, 22, 37, 56, 68,109,103, 77],
        [ 24, 35, 55, 64, 81,104,113, 92],
        [ 49, 64, 78, 87,103,121,120,101],
        [ 72, 92, 95, 98,112,100,103, 99],
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

    // mcu[id][block][h][w]
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
        let mut bitplanes: Vec<Vec<Block>> = vec![vec![Default::default(); 1024]; 3];
    
        // split bitplanes into block
        for y in 0..256 {
            for x in 0..256 {
                //println!("x={}, y={}, x/7={}, y/7={}, x%7={}, y%7={}, index{}",x,y,x/8,y/8,x%8,y%8,x/8 + 32*(y/8));
                bitplanes[0][(x / 8) + 32 * (y / 8)][x % 8][y % 8]  = Y_plane[x + 256 * y];
            }
        }
        for y in 0..256 {
            for x in 0..256 {
                bitplanes[1][(x / 8) + 32 * (y / 8)][x % 8][y % 8]  = Cb_plane[x + 256 * y];
            }
        }
        for y in 0..256 {
            for x in 0..256 {
                bitplanes[2][(x / 8) + 32 * (y / 8)][x % 8][y % 8] = Cr_plane[x + 256 * y];
            }
        }
        bitplanes
    }
    
    fn dct(in_mcu: &mut Vec<Vec<Block>>) -> Vec<Vec<Block>> {
        for id in 0..3 {
            for nblock in 0..1024 {
                let mut tmp: [[f32; 8]; 8] = Default::default();
                for i in 0..8 {
                    for j in 0..8 {
                        for x in 0..8 {
                            for y in 0..8 {
                                // inverse discrete cosine trasform
                                let i_cos = ((2*i+1) as f32 * PI / 16.0 * x as f32).cos();
                                let j_cos =((2*j+1) as f32 * PI / 16.0 * y as f32).cos();
                                tmp[i][j] += cc(x, y) * in_mcu[id][nblock][x][y] * i_cos * j_cos;
                            }
                        }
                        tmp[i][j] /= 4.0;
                    }
                in_mcu[id][nblock] = tmp;
                }
            }
        }
        in_mcu.to_vec()
    }
    fn zigzag(in_mcu: &mut Vec<Vec<Block>>) -> Vec<Vec<Block>> {
        for id in 0..3 {
            for block in 0..1024 {
                let mut tmp: Block = Default::default();
                for i in 0..8 {
                    for j in 0..8 {
                        let pos = ZZ[i][j];
                        tmp[i][j] = in_mcu[id][block][pos/8][pos%8];
                    }
                }
                //println!("{:?}",tmp);
                in_mcu[id][block] = tmp;
            }
        }
        in_mcu.to_vec()
    }
    fn quantilize() {

    }
    // show Vec<Vec<block>>
    fn display(in_mcu: &Vec<Vec<Block>>) {
        let m = ["Y", "Cb", "Cr"];
        for id in 0..3 {
            for nblock in 0..1024{
                println!("------- {} 顏色分量 {} -------", m[id], nblock);
                let block = in_mcu[id][nblock];
                for i in 0..8 {
                    for j in 0..8 {
                        print!("{:.3} ", block[i][j]);
                    }
                    println!("");
                }
            }
        }
    }
    pub fn compress(stream_vec: &Vec<u8>) {
        let colorYCbCr = read_vec8_to_color(&stream_vec);
        let mut my_mcus = read_color_to_mcus(&colorYCbCr);
        let mut after_DCT = dct(&mut my_mcus);
        // display(&after_DCT);
        let after_ZZ = zigzag(&mut after_DCT);
        display(&after_ZZ);
    }
}
