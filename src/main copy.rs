extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::{thread_rng, Rng};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const BLOCK_SIZE: u32 = 20;

struct Snake {
    body: Vec<(u32, u32)>,
    direction: Direction,
    last_tail: Option<(u32, u32)>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Snake {
    fn new() -> Snake {
        Snake {
            body: vec![
                (2 * BLOCK_SIZE, 2 * BLOCK_SIZE),
                (2 * BLOCK_SIZE, 3 * BLOCK_SIZE),
            ],
            direction: Direction::Down,
            last_tail: None,
        }
    }

    fn head(&self) -> (u32, u32) {
        self.body[0]
    }

    fn move_forward(&mut self) {
        let (x, y) = self.head();
        let (dx, dy) = match self.direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        let new_head = (
            (x as i32 + dx * BLOCK_SIZE as i32) as u32,
            (y as i32 + dy * BLOCK_SIZE as i32) as u32,
        );
        self.body.insert(0, new_head);
        self.last_tail = self.body.pop();
    }

    fn overlaps_tail(&self) -> bool {
        let head = self.head();
        for &block in self.body.iter().skip(1) {
            if head == block {
                return true;
            }
        }
        false
    }

    fn increase_size(&mut self) {
        if let Some(tail) = self.last_tail {
            self.body.push(tail);
            self.last_tail = None;
        }
    }

    fn draw(&self, context: &Context, gfx: &mut G2d) {
        let block_color: [f32; 4] = [0.0, 0.5, 0.0, 1.0];
        let block = rectangle::square(0.0, 0.0, BLOCK_SIZE as f64);
        for &(x, y) in &self.body {
            let x = x as f64;
            let y = y as f64;
            rectangle(block_color, block, context.transform.trans(x, y), gfx);
        }
    }
}

struct Game {
    snake: Snake,
    food: (u32, u32),
}

impl Game {
    fn new() -> Game {
        Game {
            snake: Snake::new(),
            food: Game::generate_food_position(),
        }
    }

    fn generate_food_position() -> (u32, u32) {
        let mut rng = thread_rng();
        let x = rng.gen_range(0..WIDTH / BLOCK_SIZE) * BLOCK_SIZE;
        let y = rng.gen_range(0..HEIGHT / BLOCK_SIZE) * BLOCK_SIZE;
        (x, y)
    }

    fn draw_food(&self, context: &Context, gfx: &mut G2d) {
        let food_color: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        let food = rectangle::square(0.0, 0.0, BLOCK_SIZE as f64);
        let (x, y) = self.food;
        let x = x as f64;
        let y = y as f64;
        rectangle(food_color, food, context.transform.trans(x, y), gfx);
    }

    fn eat_food(&mut self) {
        if self.snake.head() == self.food {
            self.snake.increase_size();
            self.food = Game::generate_food_position();
        }
    }

    fn update(&mut self) -> bool {
        self.snake.move_forward();
        if self.snake.overlaps_tail() {
            return false;
        }
        self.eat_food();
        true
    }

    fn handle_event(&mut self, event: &Event) {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            let new_direction = match key {
                Key::Up if self.snake.direction != Direction::Down => Direction::Up,
                Key::Down if self.snake.direction != Direction::Up => Direction::Down,
                Key::Left if self.snake.direction != Direction::Right => Direction::Left,
                Key::Right if self.snake.direction != Direction::Left => Direction::Right,
                _ => self.snake.direction,
            };
            self.snake.direction = new_direction;
        }
    }

    fn draw(&self, context: &Context, gfx: &mut G2d) {
        clear([1.0; 4], gfx);
        self.snake.draw(context, gfx);
        self.draw_food(context, gfx);
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new();

    let mut last_update_time = std::time::Instant::now();

    while let Some(event) = window.next() {
        let elapsed_time = last_update_time.elapsed();
        if elapsed_time < std::time::Duration::from_millis(1000 / 20) {
            continue;
        }

        last_update_time = std::time::Instant::now();

        game.handle_event(&event);

        if let Some(args) = event.render_args() {
            window.draw_2d(&event, |context, graphics, _| {
                game.draw(&context, graphics);
            });
        }

        if let Some(_) = event.update_args() {
            if !game.update() {
                break;
            }
        }
    }
}
