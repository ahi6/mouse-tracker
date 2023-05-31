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
        let delta = previous_pos.distance(&current_pos);

        // Update values
        self.stats.position = current_pos;
        self.stats.total_distance += delta;
        self.stats.delta = delta;
        self.stats.avg_speed = self.stats.avg_speed.calculate_average(&delta);
        if delta.is_normal() {
            self.stats.avg_nonzero_speed = self.stats.avg_nonzero_speed.calculate_average(&delta);
        }
    }
}

#[derive(Clone, Debug, Default)]

pub struct MouseStats {
    // "persistent" TODO: Make its own struct
    total_distance: f64,        // Total distance in pixels
    avg_speed: Average,         // Total average speed
    avg_nonzero_speed: Average, // Total average speed (excluding speed 0)
    // % of time spent moving?
    // "current" TODO: Make its own struct
    position: Point,
    delta: f64, // Current "speed/distance"
                // there's room for more..
}

impl MouseStats {
    fn new(current_pos: Point) -> Self {
        MouseStats {
            position: current_pos,
            total_distance: 0.0,
            delta: 0.0,
            avg_speed: Average::default(),
            avg_nonzero_speed: Average::default(),
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

// Maybe make this struct generic? if it's possible?
#[derive(Debug, Clone, Copy, Default)]
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
