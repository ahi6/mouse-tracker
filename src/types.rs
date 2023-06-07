#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn distance(&self, other: &Point) -> f64 {
        // Pythagorean theorem
        let dx = (other.x - self.x) as f64;
        let dy = (other.y - self.y) as f64;
        f64::sqrt(dx * dx + dy * dy)
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[x: {}, y: {}]", self.x, self.y)
    }
}

impl From<mouse_rs::types::Point> for Point {
    fn from(value: mouse_rs::types::Point) -> Self {
        Point {
            x: value.x,
            y: value.y,
        }
    }
}
#[derive(Debug, Clone, Copy, Default, serde::Deserialize, serde::Serialize)]
pub struct Average {
    pub average: f64, // The actual value
    count: i32,       // How many items is this an average of
}

impl Average {
    pub fn calculate_average(&self, new_value: &f64) -> Self {
        let new_count = self.count + 1;
        let new_average = (self.average * self.count as f64 + new_value) / new_count as f64;
        Average {
            average: new_average,
            count: new_count,
        }
    }
}

impl std::fmt::Display for Average {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.average.round() as u16)
    }
}
