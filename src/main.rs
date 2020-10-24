extern crate piston_window;
extern crate rand;

use std::time;

use piston_window::*;
use rand::prelude::*;

const WIDTH: i32 = 512;
const HEIGHT: i32 = 512;

const UNIT: i32 = 8;
const CENTER: [i32;2] = [UNIT, HEIGHT - UNIT];

const UP: [i32;2] = [0, -UNIT];
const RIGHT: [i32;2] = [UNIT, 0];
const LEFT: [i32;2] = [-UNIT, 0];
const DOWN: [i32;2] = [0, UNIT];

const TURN_DURATION: time::Duration = time::Duration::from_millis(60);

struct Body {
    pos: [i32; 2],
    dir: [i32; 2]
}

struct Snake {
    bodies: Vec<Body>
}

impl Snake {
    fn new(length: i32) -> Snake {
        let mut bodies: Vec<Body> = Vec::new();

        for i in 0..length {
            bodies.push(Body {
                pos: [CENTER[0], CENTER[1] + (i * UNIT)],
                dir: UP
            });
        }

        Snake {
            bodies: bodies
        }
    }

    fn _move (&mut self, create_tail: bool) {
        let mut new_dir = [-1, -1];
        let mut last_pos = [-1, -1];

        for body in self.bodies.iter_mut() {

            last_pos = body.pos;
            
            body.pos[0] += body.dir[0];
            body.pos[1] += body.dir[1];
            
            let tmp: [i32; 2] = body.dir;
            if new_dir[0] != -1 && new_dir[1] != -1 {
                body.dir = new_dir;
            }
            new_dir = tmp;
        }

        if create_tail {
            self.bodies.push(Body {
                pos: last_pos,
                dir: new_dir
            });
        }
    }
}

struct Entity {
    pos: [i32; 2]
}

struct Game {
    snake: Snake,
    items: Vec<Entity>,
    score: i32,
    is_game_over: bool
}

impl Game {
    fn create () -> Game {
        Game {
            snake: Snake::new(3),
            items: Game::_generate_random_items(),
            score: 0,
            is_game_over: false
        }
    }

    fn update (&mut self) {
        let mut create_tail = false;

        let mut delete_idx = 0;

        for (idx, item) in self.items.iter_mut().enumerate() {
            if self.snake.bodies[0].pos[0] == item.pos[0] && self.snake.bodies[0].pos[1] == item.pos[1] {
                create_tail = true;
                delete_idx = idx;
            }
        }

        if create_tail {
            self.score += 1;
            self.items.remove(delete_idx);
        }

        if self.snake.bodies[0].pos[0] < 0 || WIDTH <= self.snake.bodies[0].pos[0] || self.snake.bodies[0].pos[1] < 0 || HEIGHT <= self.snake.bodies[0].pos[1] {
            self.is_game_over = true
        } else {
            if self.items.len() == 0 {
                self.items = Game::_generate_random_items();
            }
    
            self.snake._move(create_tail);
        }
    }

    fn keypress (&mut self, button: Button) {
        if button == Button::Keyboard(Key::Up) {
            self.snake.bodies[0].dir = UP;
        } else if button == Button::Keyboard(Key::Right) {
            self.snake.bodies[0].dir = RIGHT;
        } else if button == Button::Keyboard(Key::Left) {
            self.snake.bodies[0].dir = LEFT;
        } else if button == Button::Keyboard(Key::Down) {
            self.snake.bodies[0].dir = DOWN;
        } else if button == Button::Keyboard(Key::R) {
            *self = Game::create();
        }
    }

    fn _generate_random_items() -> Vec<Entity> {
        let num = rand::thread_rng().gen_range(5, 15);

        let mut items: Vec<Entity> = Vec::new();

        const MIN: i32 = 0;
        const MAX_X: i32 = WIDTH / UNIT;
        const MAX_Y: i32 = HEIGHT / UNIT;

        for _ in 0..num {
            let factor_x = rand::thread_rng().gen_range(MIN, MAX_X);
            let factor_y = rand::thread_rng().gen_range(MIN, MAX_Y);

            items.push(Entity {
                pos: [UNIT * factor_x, UNIT * factor_y]
            })
        }

        items
    }
}

fn main() {
    let mut current_time = time::Instant::now();

    let mut game = Game::create();

    // create window
    let mut window: PistonWindow = WindowSettings::new("Snake", [WIDTH  as u32, HEIGHT as u32]).exit_on_esc(true).build().unwrap();

    // load font
    let mut glyphs = window.load_font("./fonts/retro_gaming.ttf").unwrap();

    while let Some(e) = window.next() {
        if let Some(button) = e.press_args() {
            game.keypress(button);
        }

        if current_time.elapsed() > TURN_DURATION && !game.is_game_over {
            current_time = time::Instant::now();
            game.update();
        }

        let g = &game;

        window.draw_2d(&e, |ctx, gl, device| {
            clear([0.0, 0.0, 0.0, 1.0], gl);

            for item in g.items.iter() {
                rectangle([1.0, 0.0, 0.0, 1.0], [item.pos[0] as f64, item.pos[1] as f64, UNIT as f64, UNIT as f64], ctx.transform, gl);
            }

            for body in g.snake.bodies.iter() {
                rectangle([0.0, 1.0, 0.0, 1.0], [body.pos[0] as f64, body.pos[1] as f64, UNIT as f64, UNIT as f64], ctx.transform, gl);
            }
            
            if g.is_game_over {
                let blue = [1.0, 1.0, 1.0, 1.0];
                text(blue, 14, "Game over", &mut glyphs, ctx.transform.trans(10.0, 10.0), gl).unwrap();
                text(blue, 10, "r - restart", &mut glyphs, ctx.transform.trans(10.0, 22.0), gl).unwrap();
                text(blue, 10, "esc - exit", &mut glyphs, ctx.transform.trans(10.0, 30.0), gl).unwrap();
            } else {
                let score = format!("score : {}", g.score);
                text([0.0, 0.0, 1.0, 1.0], 10, &score, &mut glyphs, ctx.transform.trans(100.0, 100.0), gl).unwrap();
            }
            
            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);
        });
    }
}
