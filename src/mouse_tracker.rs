use mouse_rs::Mouse;

pub struct MouseTracker {
    mouse: Mouse,
    pub stats: MouseStats,
    // sender
}

impl MouseTracker {
    pub fn new(/*sender */) -> Self {
        MouseTracker {
            mouse: Mouse::new(),
            stats: MouseStats::default(),
        }
    }
    pub fn update(&mut self) {
        let pos = self.mouse.get_position().unwrap();
        let x = pos.x;
        let y = pos.y;
        self.stats.current_pos = (x, y);
    }
}

#[derive(Clone, Debug)]

pub struct MouseStats {
    // "persistent" TODO: Make its own struct
    // total_distance: i32, // Total distance in pixels
    // avg_speed: f32,      // Total average speed (excluding speed 0)
    // "current" TODO: Make its own struct
    current_pos: (i32, i32), // TODO: Make my own damn type
                             // current_speed: f32,
                             // there's room for more..
}

impl Default for MouseStats {
    fn default() -> Self {
        MouseStats {
            // total_distance: 0,
            // avg_speed: 0.0,
            current_pos: (0, 0), // todo: replace this with a new() method instead
                                 // current_speed: 0.0,
        }
    }
}
