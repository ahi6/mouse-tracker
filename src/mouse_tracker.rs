use crate::types::*;

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
    // "persistent"
    pub total_distance: f64,        // Total distance in pixels
    pub avg_speed: Average,         // Total average speed
    pub avg_nonzero_speed: Average, // Total average speed (excluding speed 0)
    // % of time spent moving?
    // "current"
    pub position: Point,
    pub delta: f64, // Current "speed/distance"
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
