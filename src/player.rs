use crate::vector::Vector2;

pub struct Player
{
    pos: Vector2,
    dir: Vector2,

    linear_acc: f64,
    angular_acc: f64,
}

impl Player
{
    // (dx, dy) -> Must Be Normalized
    pub fn new(x: f64, y: f64, dx: f64, dy: f64, linear_acc: f64, angular_acc: f64) -> Self
    {
        let mut dir = Vector2::new(dx, dy);
        if dir.magnitude() != 1.00
        {
            dir.normalize();
        }

        Self
        {
            pos: Vector2::new(x, y),
            dir,

            linear_acc,
            angular_acc,
        }
    }

    pub fn update()
    {

    }

    fn dir(&mut self, d: Vector2)
    {
        self.dir = d;
    }
}
