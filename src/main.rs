extern crate piston_window;
extern crate rand;

mod fruit;
mod snake;

use fruit::Fruit;
use piston_window::*;
use rand::{thread_rng, Rng};
use snake::{Direction, Snake};
use std::time::Instant;

fn main() {
    let mut snake = Snake::new();
    let mut fruit = Fruit::new();

    let mut window: PistonWindow = WindowSettings::new("Snake Game", [400, 400])
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

        if snake.check_collision(&fruit) {
            let mut rng = thread_rng();
            fruit.x = rng.gen_range(0..40) as f64 * 10.0;
            fruit.y = rng.gen_range(0..40) as f64 * 10.0;
            snake.body.push(snake.body[snake.body.len() - 1]);
            score += 1;
        }

        if snake.check_body_collision() {
            score = 0;
            snake.body.truncate(3);
        }

        let now = Instant::now();
        let delta_time = now.duration_since(last_update_time);
        if delta_time.as_secs_f64() > 0.1 {
            snake.update_snake(400.0, 400.0);
            last_update_time = now;
        }

        window.draw_2d(&event, |context, graphics, device| {
            clear([1.0; 4], graphics);
            snake.draw(context, graphics);
            fruit.draw(context, graphics);

            text::Text::new_color([0.0, 0.0, 0.0, 1.0], 16)
                .draw(
                    &format!("Score: {}", score),
                    &mut glyphs,
                    &context.draw_state,
                    context.transform.trans(300.0, 370.0),
                    graphics,
                )
                .unwrap();
            glyphs.factory.encoder.flush(device);
        });
    }
}
