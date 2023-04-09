use crate::Fruit;
use piston_window::*;

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    pub body: Vec<[f64; 2]>,
    pub direction: Direction,
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            body: vec![[50.0, 50.0], [60.0, 50.0], [70.0, 50.0]],
            direction: Direction::Right,
        }
    }

    pub fn draw(&self, context: Context, graphics: &mut G2d) {
        for body_part in &self.body {
            rectangle(
                [0.0, 1.0, 0.0, 1.0], // verde
                [body_part[0], body_part[1], 10.0, 10.0],
                context.transform,
                graphics,
            );
        }
    }

    pub fn update_snake(&mut self, width: f64, height: f64) {
        let mut new_head = self.body[0];
        match self.direction {
            Direction::Up => new_head[1] -= 10.0,
            Direction::Down => new_head[1] += 10.0,
            Direction::Left => new_head[0] -= 10.0,
            Direction::Right => new_head[0] += 10.0,
        }
        // Check if the snake's head goes beyond the edge of the window
        if new_head[0] < 0.0 {
            new_head[0] = width - 10.0;
        } else if new_head[0] > width - 10.0 {
            new_head[0] = 0.0;
        }
        if new_head[1] < 0.0 {
            new_head[1] = height - 10.0;
        } else if new_head[1] > height - 10.0 {
            new_head[1] = 0.0;
        }

        self.body.pop();
        self.body.insert(0, new_head);
    }

    pub fn check_collision(&self, fruit: &Fruit) -> bool {
        let snake_head: [f64; 2] = self.body[0];
        let dx = snake_head[0] - fruit.x;
        let dy = snake_head[1] - fruit.y;
        let distance = (dx * dx + dy * dy).sqrt();
        distance < 10.0
    }

    pub fn check_body_collision(&self) -> bool {
        let snake_head: [f64; 2] = self.body[0];

        for body_part in self.body[1..].to_vec() {
            if snake_head == body_part {
                return true;
            }
        }
        false
    }
}
