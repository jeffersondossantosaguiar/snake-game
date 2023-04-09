extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::{thread_rng, Rng};
use std::time::Instant;

struct Fruit {
    x: f64,
    y: f64,
}

impl Fruit {
    fn new() -> Fruit {
        Fruit { x: 200.0, y: 200.0 }
    }

    fn draw(&self, context: Context, graphics: &mut G2d) {
        ellipse(
            [1.0, 0.0, 0.0, 1.0], // vermelho
            [self.x, self.y, 10.0, 10.0],
            context.transform,
            graphics,
        );
    }
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Snake {
    body: Vec<[f64; 2]>,
    direction: Direction,
}

impl Snake {
    fn new() -> Snake {
        Snake {
            body: vec![[50.0, 50.0], [60.0, 50.0], [70.0, 50.0]],
            direction: Direction::Right,
        }
    }

    fn draw(&self, context: Context, graphics: &mut G2d) {
        for body_part in &self.body {
            rectangle(
                [0.0, 1.0, 0.0, 1.0], // verde
                [body_part[0], body_part[1], 10.0, 10.0],
                context.transform,
                graphics,
            );
        }
    }

    fn update_snake(&mut self, width: f64, height: f64) {
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
}

fn check_collision(snake_head: [f64; 2], fruit: &Fruit) -> bool {
    let dx = snake_head[0] - fruit.x;
    let dy = snake_head[1] - fruit.y;
    let distance = (dx * dx + dy * dy).sqrt();
    distance < 10.0
}

fn check_body_collision(snake_head: [f64; 2], snake_body: &Vec<[f64; 2]>) -> bool {
    for body_part in snake_body {
        if snake_head == *body_part {
            return true;
        }
    }
    false
}

fn main() {
    let mut snake = Snake::new();
    let mut fruit = Fruit::new();

    let mut window: PistonWindow = WindowSettings::new("Snake Game", [600, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut last_update_time = Instant::now();

    let mut score = 0;
    let mut glyphs = window.load_font("assets/FiraSans-Regular.ttf").unwrap();

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            let current_direction = snake.direction;

            snake.direction = match key {
                Key::Escape => {
                    window.set_should_close(true);
                    current_direction
                }
                Key::Up if current_direction != Direction::Down => Direction::Up,
                Key::Down if current_direction != Direction::Up => Direction::Down,
                Key::Left if current_direction != Direction::Right => Direction::Left,
                Key::Right if current_direction != Direction::Left => Direction::Right,
                _ => current_direction,
            };
        }

        if check_collision(snake.body[0], &fruit) {
            let mut rng = thread_rng();
            fruit.x = rng.gen_range(0..60) as f64 * 10.0;
            fruit.y = rng.gen_range(0..60) as f64 * 10.0;
            snake.body.push(snake.body[snake.body.len() - 1]);
            score += 1;
        }

        if check_body_collision(snake.body[0], &snake.body[1..].to_vec()) {
            snake.body.truncate(3);
        }

        let now = Instant::now();
        let delta_time = now.duration_since(last_update_time);
        if delta_time.as_secs_f64() > 0.1 {
            snake.update_snake(600.0, 600.0);
            last_update_time = now;
        }

        window.draw_2d(&event, |context, graphics, _| {
            clear([1.0; 4], graphics);
            snake.draw(context, graphics);
            fruit.draw(context, graphics);

            text::Text::new_color([0.0, 0.0, 0.0, 1.0], 24)
                .draw(
                    &format!("Score: {}", score),
                    &mut glyphs,
                    &context.draw_state,
                    context.transform.trans(500.0, 50.0),
                    graphics,
                )
                .unwrap();
        });
    }
}
