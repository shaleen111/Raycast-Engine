use std::thread::sleep;
use std::time::{Duration, Instant};

use winit::{
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use pixels::{Error, Pixels, SurfaceTexture};

mod vector;
mod environment;
mod camera;
mod player;

use environment::Environment;
use camera::Camera;


fn main()
{
    let mut input = WinitInputHelper::new();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
                                .with_title("Raycast Engine")
                                .with_inner_size(winit::dpi::LogicalSize::new(480.0, 272.0))
                                .with_resizable(false)
                                .build(&event_loop).unwrap();

    let window_size = window.inner_size();

    let main_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(240, 136, main_texture).unwrap();

    let map = vec![vec![1, 1, 1, 1, 1, 1, 1, 1],
                   vec![1, 0, 0, 0, 1, 0, 0, 1],
                   vec![1, 0, 0, 0, 1, 0, 0, 1],
                   vec![1, 0, 0, 0, 0, 0, 0, 1],
                   vec![1, 0, 1, 1, 1, 1, 0, 1],
                   vec![1, 0, 0, 0, 0, 0, 0, 1],
                   vec![1, 0, 0, 0, 0, 0, 0, 1],
                   vec![1, 1, 1, 1, 1, 1, 1, 1],];

    let env = Environment::new(8, 8, map);
    let mut cam = Camera::new(1.5, 1.5, 0.0, -1.0, 67.0, 240.0, 136.0);

    let camera_speed = 5.0;
    let camera_angular_speed = 3.0;

    let mut last_frame = Instant::now();
    let mut dt = 1.0;

    let target_dt = 1.0 / 60.0;

    event_loop.run(move |event, _, control_flow|
                        {
                            let current_frame = Instant::now();

                            dt = current_frame.duration_since(last_frame).as_secs_f64();
                            // // while dt < target_dt
                            // // {
                            // //     let sleep_time = target_dt - dt;
                            // //     sleep(Duration::from_secs_f64(sleep_time));
                            // //     current_frame = Instant::now();
                            // //     dt = current_frame.duration_since(last_frame).as_secs_f64();
                            // // }


                            println!("{:?}", 1.0 / dt);

                            if input.update(&event)
                            {
                                if input.key_released(VirtualKeyCode::Escape) || input.quit()
                                {
                                    *control_flow = ControlFlow::Exit;
                                    return;
                                }

                                if input.key_held(VirtualKeyCode::W)
                                {
                                    if env.get_object_in_map((cam.pos.x + cam.projection_plane_orientation.x * camera_speed * dt) as isize,
                                                              cam.pos.y as isize) == 0
                                                              {
                                                                  cam.pos.x += cam.projection_plane_orientation.x * camera_speed * dt;
                                                                }
                                                                if env.get_object_in_map( cam.pos.x as isize,
                                                                    (cam.pos.y + cam.projection_plane_orientation.y * camera_speed * dt) as isize) == 0
                                    {
                                        cam.pos.y += cam.projection_plane_orientation.y * camera_speed * dt;
                                    }
                                }
                                else if input.key_held(VirtualKeyCode::S)
                                {
                                    if env.get_object_in_map((cam.pos.x - cam.projection_plane_orientation.x * camera_speed * dt) as isize,
                                    cam.pos.y as isize) == 0
                                    {
                                        cam.pos.x -= cam.projection_plane_orientation.x * camera_speed * dt;
                                    }
                                    if env.get_object_in_map( cam.pos.x as isize,
                                        (cam.pos.y - cam.projection_plane_orientation.y * camera_speed * dt) as isize) == 0
                                        {
                                            cam.pos.y -= cam.projection_plane_orientation.y * camera_speed * dt;
                                        }
                                    }

                                    if input.key_held(VirtualKeyCode::A)
                                    {
                                        cam.rotate(camera_angular_speed * dt);
                                    }
                                    else if input.key_held(VirtualKeyCode::D)
                                    {
                                        cam.rotate(- camera_angular_speed * dt);
                                    }
                                }

                                if let Event::RedrawRequested(_) = event
                                {
                                    let frames = pixels.get_frame();
                                    clear(frames);
                                    cam.render_frame(&env, frames);
                                    if pixels.render().is_err()
                                    {
                                        println!("Reached Here");
                                        *control_flow = ControlFlow::Exit;
                                        return;
                                    }
                            }

                            window.request_redraw();
                            last_frame = current_frame;
                        });
                    }

fn clear(frame: &mut [u8])
{
    for pixel in frame.chunks_exact_mut(4)
    {
        pixel.copy_from_slice(&[0, 0, 0, 255]);
    }
}
