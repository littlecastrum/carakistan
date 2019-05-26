use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn offset_x(self, offset: i32) -> Point {
        self + Point { x: offset, y: 0 }
    }

    pub fn offset_y(self, offset: i32) -> Point {
        self + Point { x: 0, y: offset }
    }

    pub fn offset(self, offset: Point) -> Point {
        self + offset
    }
}

#[derive(Debug, PartialEq)]
pub enum Contains {
    DoesContain,
    DoesNotContain
}

#[derive(Debug, PartialEq)]
pub struct Window {
    pub width: i32,
    pub height: i32,
}

impl Window {
    pub fn new(width: i32, height: i32) -> Window {
        Window { width, height }
    }

    pub fn contains(&self, point: Point) -> Contains {
        let left = point.x >= 0;
        let right = point.x <= self.width;
        let top = point.y >= 0;
        let bottom = point.y <= self.height;

        if  left && right && top && bottom {
            Contains::DoesContain
        } else {
            Contains::DoesNotContain
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Window;
    use super::Point;
    use super::Contains::{DoesContain, DoesNotContain};

    #[test]
    fn new_point() {
        let point = Point::new(50, 50);
        assert_eq!(point.x, 50);
        assert_eq!(point.y, 50);
    }

    #[test]
    fn offset_point() {
        let point = Point::new(50, 50);
        let point = point.offset(Point::new(4, 2));
        assert_eq!(point.x, 54);
        assert_eq!(point.y, 52);
    }

    #[test]
    fn new_window() {
        let window = Window::new(80, 50);
        assert_eq!(window.width, 80);
        assert_eq!(window.height, 50);
    }

    #[test]
    fn contains_window() {
        let window = Window::new(80, 50);
        let inside_point = Point::new(32, 12);
        let outside_point = Point::new(-3, 32);

        assert_eq!(window.contains(inside_point), DoesContain);
        assert_eq!(window.contains(outside_point), DoesNotContain);
    }
}