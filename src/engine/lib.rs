use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::keyboard::Keycode;

use std::time::{Instant, Duration};
use std::collections::HashMap;
use std::thread::sleep;

pub mod vector;
pub mod camera;
pub mod environment;

pub trait Runnable
{
    fn update(&mut self, state: &EngineState);
    fn draw(&mut self, canvas: &mut Canvas<Window>);
}

pub struct EngineState
{
    key_held: HashMap<Keycode, bool>,

    pub dt: f64,
}

impl EngineState
{
    pub fn new() -> Self
    {
        let mut key_held: HashMap<Keycode, bool> = HashMap::new();
        key_held.insert(Keycode::W, false);
        key_held.insert(Keycode::A, false);
        key_held.insert(Keycode::S, false);
        key_held.insert(Keycode::D, false);

        Self
        {
            key_held,
            dt: 0.0,
        }
    }

    pub fn key_held(&self, k: Keycode) -> bool
    {
        *self.key_held.get(&k).unwrap()
    }

    pub fn update_key(&mut self, k: Keycode, v: bool)
    {
        self.key_held.insert(k, v);
    }
}
pub struct Engine
{
    ctx: Sdl,
    canvas: Canvas<Window>,

    state: EngineState,
}

impl Engine
{
    pub fn init(width: u32, height: u32) -> Self
    {
        let ctx = sdl2::init().unwrap();
        let video_subsystem = ctx.video().unwrap();

        let window = video_subsystem.window("rust-sdl2 demo", width, height)
                                            .position_centered()
                                            .build()
                                            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        Self
        {
            ctx,
            canvas,
            state: EngineState::new(),
        }
    }

    pub fn run<T: Runnable>(&mut self, runnable: &mut T)
    {
        let mut event_pump = self.ctx.event_pump().unwrap();

        let mut last_frame = Instant::now();
        'running: loop
        {
            let current_frame = Instant::now();
            let dt = current_frame.duration_since(last_frame).as_secs_f64();

            if dt < 1.0 / 60.0
            {
                sleep(Duration::from_secs_f64(1.0 / 60.0 - dt));
            }

            self.state.dt = dt;

            for event in event_pump.poll_iter()
            {
                if let Event::Quit{..} = event
                {
                    break 'running;
                }

                self.update_keys(&event);
            }

            runnable.update(&self.state);
            runnable.draw(&mut self.canvas);

            last_frame = current_frame;
        }
    }
}


impl Engine
{
    fn update_keys(&mut self, e: &Event)
    {
        match e {

            Event::KeyDown {keycode: Some(Keycode::W), ..} =>
            {
                println!("W pressed");
                self.state.update_key(Keycode::W, true);
            },

            Event::KeyUp {keycode: Some(Keycode::W), ..} =>
            {
                self.state.update_key(Keycode::W, false);
            },

            Event::KeyDown {keycode: Some(Keycode::A), ..} =>
            {
                self.state.update_key(Keycode::A, true);
            },

            Event::KeyUp {keycode: Some(Keycode::A), ..} =>
            {
                self.state.update_key(Keycode::A, false);
            },

            Event::KeyDown {keycode: Some(Keycode::S), ..} =>
            {
                self.state.update_key(Keycode::S, true);
            },

            Event::KeyUp {keycode: Some(Keycode::S), ..} =>
            {
                self.state.update_key(Keycode::S, false);
            },

            Event::KeyDown {keycode: Some(Keycode::D), ..} =>
            {
                self.state.update_key(Keycode::D, true);
            },

            Event::KeyUp {keycode: Some(Keycode::D), ..} =>
            {
                self.state.update_key(Keycode::D, false);
            },

            _ => {},
        }
    }
}
