mod rasterizer;
mod triangle;
use crate::rasterizer::{Rasterizer, Buffers, Primitive};

use cgmath::{Vector3, Matrix4, SquareMatrix};
use cgmath::Matrix3;
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
/*use core::ffi::c_void;
use opencv::{highgui, videoio};
use opencv::core::{Size, CV_32FC3, Mat, MatTraitManual, Mat_AUTO_STEP, CV_8UC3, MatTrait};
use opencv::videoio::VideoCaptureTrait;
use opencv::highgui::imshow;*/


/*fn run() -> opencv::Result<()> {
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
        let mut frame = Mat::default()?;
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
}*/

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
    //run();
    let angle = 0f32;
    let mut command_line = false;
    let filename = String::from("output.png");

    let argv:Vec<String> = std::env::args().collect();
    if argv.len() >= 3{
        command_line = true;

    }

    let mut r = Rasterizer::new(720,720);
    let eye_pos = Vector3::new(0f32,0f32,5f32);

    let pos = vec![Vector3::new(2f32,0f32,-2f32)
                   ,Vector3::new(0f32,2f32,-2f32),Vector3::new(-2f32,0f32,-2f32)];
    let ind = vec![Vector3::new(0,1,2)];
    let pos_id = r.load_positions(pos);
    let ind_id = r.load_indices(ind);

    let mut key = 0;
    let mut frame_count = 0;

    //sdl2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("run run",720,720)
        .position_centered()
        .vulkan()
        .build()
        .map_err(|e|e.to_string()).unwrap();
    let mut canvas = window.into_canvas().build().map_err(|e|e.to_string()).unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture=texture_creator.create_texture_streaming(PixelFormatEnum::RGB24,720,720)
        .map_err(|e|e.to_string()).unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: while key != 27 {
        r.clear(Buffers::Color);
        r.clear(Buffers::Depth);
        r.set_model(get_model_matrix(angle));
        r.set_view(get_view_matrix(&eye_pos));
        r.set_projection(get_projection_matrix(45.0,1.0,0.1,50.0));

        r.draw(&pos_id,&ind_id,Primitive::Triangle);

        texture.with_lock(None,|buffer,pitch|{
            let frame_buf = r.frame_buffer();
           for y in 0..720{
               let row_offset = y*pitch;
               let frame_row_offset = y*720;
               for x in 0..720{
                   let offset = row_offset + x*3;
                   let frame_color = frame_buf[frame_row_offset+x];
                   buffer[offset] = frame_color.x as u8;
                   buffer[offset + 1] = frame_color.y as u8;
                   buffer[offset + 2] = frame_color.z as u8;
               }
           }
        });

        canvas.clear();
        canvas.copy(&texture,None,None);
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..}
                    |Event::KeyDown {keycode:Some(Keycode::Escape),..}=>{
                    break 'running
                },
                _ =>{}
            }
        }
        /*
        let mut image;
        //let mut oimage:Mat = Mat::default().unwrap();
        unsafe {
            image = Mat::new_rows_cols_with_data(720, 720, CV_32FC3, r.frame_buffer().as_mut_ptr() as *mut c_void, Mat_AUTO_STEP).unwrap();
        }

        //image.convert_to(&mut oimage,CV_8UC3,0.09,0.0);

        highgui::imshow("run run",&mut image);


        key = highgui::wait_key(10).unwrap();*/


        frame_count+=1;
        println!("frame count: {}",frame_count);
    }

    /*let mut inputArg = String::new();
    std::io::stdin().read_line(&mut inputArg);*/

    println!("{:?}",argv);
}