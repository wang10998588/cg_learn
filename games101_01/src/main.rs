mod rasterizer;
mod triangle;

use cgmath::Vector3;
use cgmath::Matrix3;

/*
fn run() -> opencv::Result<()> {
    let window = "video capture";
    highgui::named_window(window, 1)?;
    #[cfg(feature = "opencv-32")]
    let mut cam = videoio::VideoCapture::new_default(0)?;  // 0 is the default camera
    #[cfg(not(feature = "opencv-32"))]
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;  // 0 is the default camera
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Unable to open default camera!");
    }
    loop {
        let mut frame = core::Mat::default()?;
        cam.read(&mut frame)?;
        if frame.size()?.width > 0 {
            highgui::imshow(window, &mut frame)?;
        }
        let key = highgui::wait_key(10)?;
        if key > 0 && key != 255 {
            break;
        }
    }
    Ok(())
}
*/

fn main() {
    let angle = 0f32;
    let mut command_line = false;
    let filename = String::from("output.png");

    let argv:Vec<String> = std::env::args().collect();
    if argv.len() >= 3{
        command_line = true;

    }



    /*let mut inputArg = String::new();
    std::io::stdin().read_line(&mut inputArg);*/

    println!("{:?}",argv);
}