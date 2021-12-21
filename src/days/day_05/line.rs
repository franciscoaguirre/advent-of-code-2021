use super::Point;

#[derive(Debug, PartialEq)]
pub enum LineDirection {
    Horizontal,
    Vertical,
    Diagonal,
}

#[derive(Debug, PartialEq)]
pub struct Line {
    pub start: Point,
    pub end: Point,
    pub direction: LineDirection,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Line {
        Line {
            start: if start < end { start } else { end },
            end: if start < end { end } else { start },
            direction: if start.y == end.y {
                LineDirection::Horizontal
            } else if start.x == end.x {
                LineDirection::Vertical
            } else {
                LineDirection::Diagonal
            },
        }
    }

    pub fn from_string(string: &str) -> Line {
        let parts: Vec<&str> = string.split(" -> ").collect();
        let start = Point::from_string(parts[0]);
        let end = Point::from_string(parts[1]);
        Line::new(start, end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn building_from_string() {
        let string = "3,2 -> 2,8";
        let (start, end) = (Point::new(3, 2), Point::new(2, 8));
        assert_eq!(Line::from_string(string), Line::new(start, end));
    }
}
