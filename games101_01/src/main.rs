mod rasterizer;
mod triangle;

use opencv::{highgui,core};
use cgmath::{Vector3, Matrix4, SquareMatrix};
use cgmath::Matrix3;
use crate::rasterizer::{Rasterizer, Buffers, Primitive};
use opencv::core::{Size, CV_32FC3, Mat};

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
fn get_view_matrix(eye_pos:&Vector3<f32>)->Matrix4<f32>{
    let mut view = Matrix4::identity();
    let mut translate = Matrix4::new(
        1f32, 0f32, 0f32, -eye_pos[0],
        0f32, 1f32, 0f32, -eye_pos[1],
        0f32, 0f32, 1f32, -eye_pos[2],
        0f32, 0f32, 0f32, 1f32,
    );
    view = translate*view;
    view
}

fn get_model_matrix(rotation_angle:f32)->Matrix4<f32>{
    let mut model = Matrix4::identity();

    // TODO: Implement this function
    // Create the model matrix for rotating the triangle around the Z axis.
    // Then return it.

    model
}

fn get_projection_matrix(eye_fov:f32,aspect_ratio:f32,zNear:f32,zFar:f32)->Matrix4<f32>{
    let mut projection = Matrix4::identity();

    // TODO: Implement this function
    // Create the projection matrix for the given parameters.
    // Then return it.

    projection
}


fn main() {
    let angle = 0f32;
    let mut command_line = false;
    let filename = String::from("output.png");

    let argv:Vec<String> = std::env::args().collect();
    if argv.len() >= 3{
        command_line = true;

    }

    let mut r = Rasterizer::new(700,700);
    let eye_pos = Vector3::new(0f32,0f32,5f32);
    let pos = vec![Vector3::new(2f32,0f32,-2f32)
                   ,Vector3::new(0f32,2f32,-2f32),Vector3::new(-2f32,0f32,-2f32)];
    let ind = vec![Vector3::new(0,1,2)];
    let pos_id = r.load_positions(pos);
    let ind_id = r.load_indices(ind);

    let mut key = 0;
    let mut frame_count = 0;

    while key != 27 {
        //r.clear(Buffers::Color|Buffers::Depth);
        r.set_model(get_model_matrix(angle));
        r.set_view(get_view_matrix(&eye_pos));
        r.set_projection(get_projection_matrix(45.0,1.0,0.1,50.0));

        r.draw(&pos_id,&ind_id,Primitive::Triangle);

        //core::Mat::new_size_with_data(Size::new(700,700),CV_32FC3,r.frame_buffer())
        let mut image = Mat::default().unwrap();

        highgui::imshow("",&mut image);

        frame_count+=1;
        println!("frame count: {}",frame_count);

        key = highgui::wait_key(10).unwrap();

    }
    /*let mut inputArg = String::new();
    std::io::stdin().read_line(&mut inputArg);*/

    println!("{:?}",argv);
}