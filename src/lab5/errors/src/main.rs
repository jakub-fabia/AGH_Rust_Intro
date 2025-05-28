struct Rectangle {
    width : f64,
    height : f64
}
 
impl Rectangle {
    fn new(width: f64, height: f64) -> Result<Rectangle, String> {
        if width < 0.0 || height < 0.0 {
            return Err("Rectangle cannot have negative width or height".to_string());
        }
        Ok(Rectangle { width, height })
    }
 
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn bigger(w1: f64, h1 : f64, w2 : f64, h2 : f64) -> Result<Rectangle, String> {
        let r1 = Rectangle::new(w1, h1)?;
        let r2 = Rectangle::new(w2, h2)?;
    
        if r1.area() > r2.area() {
            Ok(r1)
        } else {
            Ok(r2)
        }
    }

    fn read_from_file(path : &str) -> Result<Rectangle, ...> {
        let s = fs::read_to_string(path)?;
        let mut iter = s.split_whitespace();
        let width : f64 = iter.next().ok_or("Cannot convert string to width".to_string())?.parse()?;
        let height : f64 = iter.next().ok_or("Cannot convert string to height".to_string())?.parse()?;
    
        Rectangle::new(width, height)
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        const DEFAULT_WIDTH: f64 = 1.0;
        const DEFAULT_HEIGHT: f64 = 1.0;
        Rectangle{ width : DEFAULT_WIDTH, height : DEFAULT_HEIGHT }
    }
}
 
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_rectangle() {
        // given
        let width = 4.5;
        let height = 5.7;
 
        // when
        let r = Rectangle::new(width, height).unwrap();
 
        // then
        assert!((r.width - width).abs() < f64::EPSILON && (r.height - height).abs() < f64::EPSILON);
    }
 
    #[test]
    fn test_new_rectangle_with_negative() {
        let r = Rectangle::new(-1.0, 1.0);
        match r {
            Err(s) => assert_eq!("Rectangle cannot have negative width or height", s.as_str()),
            Ok(_) => panic!() // the result shouldn't be Ok
        }
    }

    #[test]
    #[should_panic]
    fn test_unwrap() {
        // that is ok
        let r = Rectangle::new(1.0, 2.0);
        let rec = r.unwrap(); // consumes the value
    
        // that generates panic
        let r = Rectangle::new(-1.0, 2.0);
        let rec = r.unwrap(); // consumes the value
    
    }
    
    #[test]
    #[should_panic]
    fn test_expect() {
        // that is ok
        let r = Rectangle::new(1.0, 2.0);
        let rec = r.expect("This should be always a proper rectangle");
    
        // that generates panic
        let r = Rectangle::new(1.0, -2.0);
        let rec = r.expect("Don't do that!");
    }

    #[test]
    fn test_is_ok() {
        let r = Rectangle::new(1.0, 2.0);
        assert!(r.is_ok());
        assert!(!r.is_err());
    }
    #[test]
    fn test_unwrap_or_else() {
        const DEFAULT_WIDTH: f64 = 1.0;
        const DEFAULT_HEIGHT: f64 = 1.0;
        let r = Rectangle::new(-1.0, -2.0);
        let rec = r.unwrap_or_else(|_| Rectangle::new(DEFAULT_WIDTH, DEFAULT_HEIGHT).unwrap());
    
        assert!((rec.width - DEFAULT_WIDTH).abs() < f64::EPSILON && (rec.height - DEFAULT_HEIGHT).abs() < f64::EPSILON);
    }
    #[test]
    fn test_unwrap_or_default() {
        const DEFAULT_WIDTH: f64 = 1.0;
        const DEFAULT_HEIGHT: f64 = 1.0;
        let r = Rectangle::new(-1.0, -2.0);
        let rec = r.unwrap_or_default();
    
        assert!((rec.width - DEFAULT_WIDTH).abs() < f64::EPSILON && (rec.height - DEFAULT_HEIGHT).abs() < f64::EPSILON);
    
        let r = Rectangle::new(1.0, 2.0);
        let rec = r.unwrap_or_default();
        assert!((rec.width - 1.0).abs() < f64::EPSILON && (rec.height - 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_bigger() {
        // given
        let w1 = 1.0;
        let h1 = 2.0;
        let w2 = 3.0;
        let h2 = 4.0;
    
        // when
        let r = Rectangle::bigger(w1, h1, w2, h2);
    
        // then
        assert!((r.unwrap().area() - (w2 * h2)).abs() < f64::EPSILON);
    }
    
    #[test]
    fn test_bigger_err() {
        // given
        let w1 = 1.0;
        let h1 = -2.0; // wrong width
        let w2 = 3.0;
        let h2 = 4.0;
    
        // when
        let r = Rectangle::bigger(w1, h1, w2, h2);
    
        // then
        assert!(r.is_err());
    }
}
 
fn main() {
    // nothing is here
}