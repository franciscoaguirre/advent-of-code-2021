use std::cmp::{Ordering, PartialOrd};
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    pub fn from_string(string: &str) -> Point {
        let parts: Vec<&str> = string.split(',').collect();
        let x: usize = parts[0].parse().unwrap();
        let y: usize = parts[1].parse().unwrap();
        Point::new(x, y)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.y.cmp(&other.y) {
            Ordering::Equal => Some(self.x.cmp(&other.x)),
            ordering => Some(ordering),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition_works() {
        let sum = Point::new(5, 1) + Point::new(3, 4);
        assert_eq!(sum, Point::new(8, 5));
    }

    #[test]
    fn ordering_works() {
        let small = Point::new(0, 0);
        let medium = Point::new(2, 1);
        let big = Point::new(1, 3);

        assert!(big > small);
        assert!(big > medium);
        assert!(medium > small);
    }

    #[test]
    fn building_from_string() {
        let string = "0,4";
        assert_eq!(Point::from_string(string), Point::new(0, 4));
    }
}
