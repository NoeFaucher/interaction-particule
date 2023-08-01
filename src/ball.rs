use gtk::cairo;





const BALL_RADIUS: f64 = 20.0;

pub struct Ball {
    x: f64,
    y: f64,
    velocity_x: f64,
    velocity_y: f64,
}

impl Ball {
    pub fn new(x: f64, y: f64, velocity_x: f64, velocity_y: f64) -> Self {
        Ball {
            x,
            y,
            velocity_x,
            velocity_y,
        }
    }

    pub fn update_position(&mut self, width: f64, height: f64) {
        self.x += self.velocity_x;
        self.y += self.velocity_y;

        // Handle collisions with the window edges
        if self.x - BALL_RADIUS < 0.0 || self.x + BALL_RADIUS > width {
            self.velocity_x *= -1.0;
        }

        if self.y - BALL_RADIUS < 0.0 || self.y + BALL_RADIUS > height {
            self.velocity_y *= -1.0;
        }
    }

    pub fn draw(&self, context: &cairo::Context) {
        context.arc(self.x, self.y, BALL_RADIUS, 0.0, 2.0 * std::f64::consts::PI);
        context.set_source_rgb(0.0, 0.0, 1.0); // Blue color
        context.fill().unwrap();
    }
}