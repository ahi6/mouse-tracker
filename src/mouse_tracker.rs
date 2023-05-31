pub struct MouseTracker {
    mouse: mouse_rs::Mouse,
    pub stats: MouseStats,
}

impl MouseTracker {
    pub fn new() -> Self {
        let mouse = mouse_rs::Mouse::new();
        MouseTracker {
            // Sets the mouse_pos based on where the mouse begins, for corectness' sake
            stats: MouseStats::new(mouse.get_position().unwrap().into()),
            mouse,
        }
    }
    pub fn update(&mut self) {
        let previous_pos = &self.stats.position;
        let current_pos: Point = self.mouse.get_position().unwrap().into();
        let current_distance: f64 = previous_pos.distance(&current_pos);

        // Update position
        self.stats.position = current_pos;
        // Update distance
        self.stats.total_distance += current_distance;
    }
}

#[derive(Clone, Debug, Default)]

pub struct MouseStats {
    // "persistent" TODO: Make its own struct
    total_distance: f64, // Total distance in pixels
    // avg_speed: f64,      // Total average speed (excluding speed 0)
    // "current" TODO: Make its own struct
    position: Point,
    // delta: f64, // Current "speed"
    // there's room for more..
}

impl MouseStats {
    fn new(current_pos: Point) -> Self {
        MouseStats {
            position: current_pos,
            total_distance: 0.0,
        }
    }
}

#[derive(Clone, Debug, Default)]
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

impl From<mouse_rs::types::Point> for Point {
    fn from(value: mouse_rs::types::Point) -> Self {
        Point {
            x: value.x,
            y: value.y,
        }
    }
}
