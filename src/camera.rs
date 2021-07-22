use std::f64::consts::{PI, FRAC_PI_2};

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

        println!("{:?}, {:?}", projection_plane.x, projection_plane.y);
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
    pub fn render_frame(&mut self, env: &Environment, frame: &mut [u8])
    {
        for x in 0..(self.width as usize)
        {
            let (side, perp_dist) = self.cast_ray(x, env);

            let mut line_height = self.height as f64 / perp_dist;

            if self.height < line_height
            {
                line_height = self.height;
            }

            let (line_start, line_end) = ((self.height - line_height) / 2.0, (self.height + line_height) / 2.0);

            let mut color = [255, 255, 255, 255];

            if side == Side::Y
            {
                color = [128, 128, 128, 128];
            }

            self.draw_vertical_line(x, line_start.round() as usize, line_end.round() as usize, &color, frame);
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
    fn cast_ray(&self, x: usize, env: &Environment) -> (Side, f64)
    {
        let projected_x = 2.0 * x as f64 / self.width as f64;
        let ray_dir = self.projection_plane_orientation + projected_x * self.projection_plane;
        let mut current_map_tile = Vector2::new(self.pos.x.trunc(), self.pos.y.trunc());

        let delta_x: f64;
        let delta_y: f64;

        // If Ray Is Going Straight Horizontally
        // There Is No Way To Reach Another Y Side
        if ray_dir.x == 0.0
        {
            delta_x = 1.0;
            delta_y = 0.0;
        }
        // If Ray Is Going Straight Vertically
        // There Is No Way To Reach Another X Side
        else if ray_dir.y == 0.0
        {
            delta_x = 0.0;
            delta_y = 1.0;
        }
        else
        {
            delta_x = (1.0 / ray_dir.x).abs();
            delta_y = (1.0 / ray_dir.y).abs();
        }

        let step_x: f64;
        let step_y: f64;

        let mut side_x: f64;
        let mut side_y: f64;

        if ray_dir.x < 0.0
        {
            step_x = -1.0;
            side_x = (self.pos.x - current_map_tile.x) * delta_x;
        }
        else
        {
            step_x = 1.0;
            side_x = (current_map_tile.x + 1.0 - self.pos.x) * delta_x;
        }

        if ray_dir.y < 0.0
        {
            step_y = -1.0;
            side_y = (self.pos.y - current_map_tile.y) * delta_y;
        }
        else
        {
            step_y = 1.0;
            side_y = (current_map_tile.y + 1.0 - self.pos.y) * delta_y;
        }

        let mut side: Side;
        loop
        {
            if side_x < side_y
            {
                side_x += delta_x;
                current_map_tile.x += step_x;
                side = Side::X;
            }
            else
            {
                side_y += delta_y;
                current_map_tile.y += step_y;
                side = Side::Y;
            }

            if env.get_object_in_map(current_map_tile.x as isize, current_map_tile.y as isize) != 0
            {
                break;
            }
        }

        let perp_dist: f64;
        if side == Side::X
        {
            perp_dist = (current_map_tile.x - self.pos.x + (1.0 - step_x) / 2.0) / ray_dir.x;
        }
        else
        {
            perp_dist = (current_map_tile.y - self.pos.y + (1.0 - step_y) / 2.0) / ray_dir.y;
        }

        return (side, perp_dist);
    }

    fn draw_vertical_line(&mut self, x: usize, y1: usize, y2: usize, color: &[u8; 4], frame: &mut [u8])
    {
        let w = self.width as usize;
        let h = self.height as usize;
        for y in 0..y1
        {
            frame[4 * y * w + 4 * x .. 4 * y * w + 4 * x + 4].copy_from_slice(&[0, 0, 0, 255]);
        }
        for y in y2..h
        {
            frame[4 * y * w + 4 * x .. 4 * y * w + 4 * x + 4].copy_from_slice(&[0, 0, 0, 255]);
        }
        for y in y1..y2
        {
            frame[4 * y * w + 4 * x .. 4 * y * w + 4 * x + 4].copy_from_slice(color);
        }
    }
}
