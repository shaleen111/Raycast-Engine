use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Vector2
{
    pub x: f64,
    pub y: f64,
}

impl Vector2
{
    pub fn new(x: f64, y: f64) -> Self
    {
        Self
        {
            x,
            y
        }
    }

    // Anticlockwise Rotation in Screen Space
    // => Equivalent to Clockwise Rotation in Cartesian Space
    pub fn acw_screen_rotate(&mut self, angle: f64)
    {
        let (c, s) = (angle.cos(), angle.sin());
        let x_ = self.x;
        self.x = c * self.x + s * self.y;
        self.y = - s * x_ + c * self.y;
    }

    pub fn magnitude(&self) -> f64
    {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }

    pub fn normalize(&mut self)
    {
        *self /= self.magnitude();
    }
}

impl ops::Add for Vector2
{
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Self::Output
    {
        Self
        {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign for Vector2
{
    fn add_assign(&mut self, rhs: Vector2)
    {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Sub for Vector2
{
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Self::Output
    {
        Self
        {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::SubAssign for Vector2
{
    fn sub_assign(&mut self, rhs: Vector2)
    {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::Mul<f64> for Vector2
{
    type Output = Vector2;

    fn mul(self, rhs: f64) -> Self::Output
    {
        Vector2
        {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::MulAssign<f64> for Vector2
{
    fn mul_assign(&mut self, rhs: f64)
    {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl ops::Mul<Vector2> for f64
{
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Self::Output
    {
        Vector2
        {
            x: self * rhs.x,
            y: self * rhs.y
        }
    }
}

impl ops::Div<f64> for Vector2
{
    type Output = Vector2;

    fn div(self, rhs: f64) -> Self::Output
    {
        Self
        {
            x: self.x / rhs,
            y: self.y /rhs,
        }
    }
}

impl ops::DivAssign<f64> for Vector2
{
    fn div_assign(&mut self, rhs: f64)
    {
        self.x /= rhs;
        self.y /= rhs;
    }
}
