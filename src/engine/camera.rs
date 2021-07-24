use std::f64::consts::{PI, FRAC_PI_2};
use sdl2::render::{Canvas, RenderTarget};
use sdl2::pixels::Color;

use crate::vector::Vector2;
use crate::environment::Environment;

#[derive(PartialEq)]
enum Side
{
    X,
    Y
}

/// A Camera That Converts 2D Environment to 3D via Ray-Casting
pub struct Camera
{
    pub pos: Vector2,

    projection_plane: Vector2,
    pub projection_plane_orientation: Vector2,

    width: f64,
    height: f64,
}

impl Camera
{

    // Returns A New Camera After Calculating Projection Plane
    // From A Given Orientation and FOV
    pub fn new(x: f64, y: f64, dx: f64, dy: f64, fov: f64, width: f64, height: f64) -> Self
    {
        let scale = (fov * PI / 360.0).tan();
        let mut projection_plane = Vector2::new(dx, dy);
        projection_plane.acw_screen_rotate(-FRAC_PI_2);
        projection_plane *= scale;

        Self
        {
            pos: Vector2::new(x, y),

            projection_plane,
            projection_plane_orientation: Vector2::new(dx, dy),

            width,
            height,
        }
    }

    // Renders a Frame
    pub fn render<T> (&mut self, env: &Environment, canvas: &mut Canvas<T>)
                      where T: RenderTarget
    {
        for x in 0..(self.width as usize)
        {
            let (obj_type, side, perp_dist) = self.cast_ray(x, env);


            let mut line_height = (self.height / perp_dist).round() as i32;

            if line_height > self.height as i32
            {
                line_height = self.height as i32;
            }
            else if line_height < 0
            {
                line_height = 0;
            }

            let mut color = env.get_resource(obj_type).unwrap();
            if side == Side::Y
            {
                color = Color::RGB(color.r / 2, color.g / 2, color.b / 2);
            }

            canvas.set_draw_color(color);

            let line_start = (x as i32, (self.height as i32 - line_height) / 2);
            let line_end = (x as i32, (self.height as i32 + line_height) / 2);
            canvas.draw_line(line_start, line_end).unwrap();
        }
    }

    // Translates The Camera In Space
    pub fn translate(&mut self, t: Vector2)
    {
        self.pos += t;
    }

    // Rotates Camera
    // And Returns New Orientation Of Camera
    pub fn rotate(&mut self, angle: f64) -> Vector2
    {
        self.projection_plane.acw_screen_rotate(angle);
        self.projection_plane_orientation.acw_screen_rotate(angle);

        return self.projection_plane_orientation;
    }
}

impl Camera
{
    // Cast Ray For A Given Column of Pixel
    // And Return Distance
    fn cast_ray(&self, x: usize, env: &Environment) -> (usize, Side, f64)
    {
        let camera_x = 2.0 * (x as f64) / self.width - 1.0;
        let ray_dir = self.projection_plane_orientation + camera_x * self.projection_plane;
        let mut map_pos = Vector2::new(self.pos.x.trunc(), self.pos.y.trunc());

        let delta_x: f64;
        let delta_y: f64;
        // if ray_dir.x == 0.0
        // {
        //     delta_x = 1.0;
        //     delta_y = 0.0;
        // }
        // else if ray_dir.y == 0.0
        // {
        //     delta_x = 0.0;
        //     delta_y = 1.0;
        // }
        // else
        // {
        //     delta_x = (1.0 / ray_dir.x).abs();
        //     delta_y = (1.0 / ray_dir.y).abs();
        // }

        delta_x = (1.0 / ray_dir.x).abs();
        delta_y = (1.0 / ray_dir.y).abs();

        let step_x: f64;
        let step_y: f64;

        let mut side_x: f64;
        let mut side_y: f64;

        if ray_dir.x < 0.0
        {
            step_x = -1.0;
            side_x = (self.pos.x - map_pos.x) * delta_x;
        }
        else
        {
            step_x = 1.0;
            side_x = (map_pos.x + 1.0 - self.pos.x ) * delta_x;
        }

        if ray_dir.y < 0.0
        {
            step_y = -1.0;
            side_y = (self.pos.y - map_pos.y) * delta_y;
        }
        else
        {
            step_y = 1.0;
            side_y = (map_pos.y + 1.0 - self.pos.y) * delta_y;
        }

        let mut side: Side;
        let mut obj_type: usize;
        loop
        {
            if side_x < side_y
            {
                side_x += delta_x;
                map_pos.x += step_x;
                side = Side::X;
            }
            else
            {
                side_y += delta_y;
                map_pos.y += step_y;
                side = Side::Y;
            }

            obj_type = env.get_object_in_map(map_pos.x as isize, map_pos.y as isize);
            if obj_type != 0
            {
                break;
            }
        }

        let perp_dist: f64;
        if side == Side::X
        {
            perp_dist = (map_pos.x - self.pos.x + (1.0 - step_x) / 2.0) / ray_dir.x;
        }
        else
        {
            perp_dist = (map_pos.y - self.pos.y + (1.0 - step_y) / 2.0) / ray_dir.y;
        }

        return (obj_type, side, perp_dist);
    }
}
