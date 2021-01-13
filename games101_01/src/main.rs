mod rasterizer;
mod triangle;

use cgmath::Vector3;
use cgmath::Matrix3;



fn main() {
    let angle = 0f32;
    let mut command_line = false;
    let filename = String::from("output.png");

    let argv:Vec<String> = std::env::args().collect();
    if argv.len() >= 3{
        command_line = true;

    }

    println!("{:?}",argv);
}