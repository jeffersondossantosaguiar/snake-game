use piston_window::*;

pub struct Fruit {
    pub x: f64,
    pub y: f64,
}

impl Fruit {
    pub fn new() -> Fruit {
        Fruit { x: 200.0, y: 200.0 }
    }

    pub fn draw(&self, context: Context, graphics: &mut G2d) {
        ellipse(
            [1.0, 0.0, 0.0, 1.0], // vermelho
            [self.x, self.y, 10.0, 10.0],
            context.transform,
            graphics,
        );
    }
}
