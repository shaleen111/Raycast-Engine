use engine::Engine;
use engine::EngineState;
use engine::Runnable;
use engine::environment::Environment;
use engine::camera::Camera;

use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::keyboard::Keycode;

pub struct Game
{
    env: Environment,
    camera: Camera,
    camera_speed: f64,
    camera_angular_speed: f64,
}

impl Game
{
    pub fn init(width: u32, height: u32) -> Self
    {
        let map: Vec<Vec<usize>> = vec![vec![2, 3, 3, 3, 3, 3, 3, 3,],
                                        vec![2, 0, 0, 0, 0, 0, 0, 3,],
                                        vec![2, 0, 1, 1, 0, 0, 0, 3,],
                                        vec![2, 0, 1, 0, 0, 0, 0, 3,],
                                        vec![2, 0, 0, 0, 1, 1, 0, 3,],
                                        vec![2, 0, 0, 0, 1, 1, 0, 3,],
                                        vec![2, 0, 0, 0, 0, 0, 0, 3,],
                                        vec![2, 2, 2, 2, 2, 2, 2, 2],];

        let mut env = Environment::new(8, 8, map);
        env.add_resource(1, Color::RGB(255, 255, 255));
        env.add_resource(2, Color::RGB(255, 0, 0));
        env.add_resource(3, Color::RGB(0, 255, 0));
        env.add_resource(4, Color::RGB(0, 0, 255));

        let camera = Camera::new(1.5, 1.5, 0.0, -1.0, 67.0,  width as f64, height as f64);
        let camera_speed = 5.0;
        let camera_angular_speed = 3.0;

        Self
        {
            env,
            camera,
            camera_speed,
            camera_angular_speed,
        }
    }
}

impl Runnable for Game
{
    fn update(&mut self, state: &EngineState)
    {
        println!("{:?}", 1.0 / state.dt);
        if state.key_held(Keycode::W)
        {
            println!("W");
            let mut new_x = self.camera.pos.x + self.camera.projection_plane_orientation.x * self.camera_speed * state.dt;
            let mut new_y = self.camera.pos.y + self.camera.projection_plane_orientation.y * self.camera_speed * state.dt;

            if new_x < 0.0
            {
                new_x = 0.0;
            }
            if new_y < 0.0
            {
                new_y = 0.0;
            }

            if new_x >= 8.0
            {
                new_x = 7.0;
            }
            if new_y >= 8.0
            {
                new_y = 7.0;
            }

            if self.env.get_object_in_map(new_x as isize, self.camera.pos.y as isize) == 0
            {
                self.camera.pos.x = new_x;
            }

            if self.env.get_object_in_map(self.camera.pos.x as isize, new_y as isize) == 0
            {
                self.camera.pos.y = new_y;
            }
        }
        else if state.key_held(Keycode::S)
        {
            let mut new_x = self.camera.pos.x - self.camera.projection_plane_orientation.x * self.camera_speed * state.dt;
            let mut new_y = self.camera.pos.y - self.camera.projection_plane_orientation.y * self.camera_speed * state.dt;

            if new_x < 0.0
            {
                new_x = 0.0;
            }
            if new_y < 0.0
            {
                new_y = 0.0;
            }

            if new_x >= 8.0
            {
                new_x = 7.0;
            }
            if new_y >= 8.0
            {
                new_y = 7.0;
            }

            if self.env.get_object_in_map(new_x as isize, self.camera.pos.y as isize) == 0
            {
                self.camera.pos.x = new_x;
            }

            if self.env.get_object_in_map(self.camera.pos.x as isize, new_y as isize) == 0
            {
                self.camera.pos.y = new_y;
            }
        }

        if state.key_held(Keycode::A)
        {
            self.camera.rotate(self.camera_angular_speed * state.dt);
        }
        else if state.key_held(Keycode::D)
        {
            self.camera.rotate(- self.camera_angular_speed * state.dt);
        }
    }

    fn draw(&mut self, canvas: &mut Canvas<Window>)
    {
        canvas.set_draw_color(Color::RGB(150, 150, 180));
        canvas.clear();

        self.camera.render(&self.env, canvas);

        canvas.present();
    }
}

pub fn main()
{
    let mut e = Engine::init(600, 480);
    let g = &mut Game::init(600, 480);
    e.run(g);
}
