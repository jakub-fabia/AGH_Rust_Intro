use std::fmt::Formatter;
use std::fmt::Display;

pub struct Vec2D {
    x : f32,
    y : f32
}

impl Display for Vec2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "2D Vector with x: {} and y: {}", self.x, self.y)?;
        Ok(())
    }
}

impl Vec2D {
    
}