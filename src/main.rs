extern crate piston_window;
extern crate rand;

use std::collections::LinkedList;
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
    bodies: LinkedList<Body>
}

impl Snake {
    fn new(length: i32) -> Snake {
        let mut bodies = LinkedList::new();

        for i in 0..length {
            bodies.push_back(Body {
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
            self.bodies.push_back(Body {
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
    items: Vec<Entity>
}

impl Game {
    fn create () -> Game {
        Game {
            snake: Snake::new(3),
            items: Game::_generate_random_items()
        }
    }

    fn update (&mut self) {
        let mut create_tail = false;

        match self.snake.bodies.front() {
            None => {},
            Some (head) => {
                let mut delete_idx = 0;

                for (idx, item) in self.items.iter_mut().enumerate() {
                    if head.pos[0] == item.pos[0] && head.pos[1] == item.pos[1] {
                        create_tail = true;
                        delete_idx = idx;
                    }
                }

                if create_tail {
                    self.items.remove(delete_idx);
                }

                if head.pos[0] < 0 || WIDTH <= head.pos[0] || head.pos[1] < 0 || HEIGHT <= head.pos[1] {
                    panic!("Game Over");
                }
            }
        }

        if self.items.len() == 0 {
            self.items = Game::_generate_random_items();
        }

        self.snake._move(create_tail);
    }

    fn keypress (&mut self, button: Button) {
        match self.snake.bodies.front_mut() {
            None => {},
            Some (x) => {
                if button == Button::Keyboard(Key::Up) {
                    x.dir = UP;
                } else if button == Button::Keyboard(Key::Right) {
                    x.dir = RIGHT;
                } else if button == Button::Keyboard(Key::Left) {
                    x.dir = LEFT;
                } else if button == Button::Keyboard(Key::Down) {
                    x.dir = DOWN;
                }
            }
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
    let mut window: PistonWindow = WindowSettings::new("Snake", [WIDTH  as u32, HEIGHT as u32]).exit_on_esc(true).build().unwrap();

    while let Some(e) = window.next() {
        if let Some(button) = e.press_args() {
            game.keypress(button);
        }

        if current_time.elapsed() > TURN_DURATION {
            current_time = time::Instant::now();
            game.update();
        }

        let snake = &game.snake;
        let items = &game.items;

        window.draw_2d(&e, |c, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            for item in items.iter() {
                rectangle([1.0, 0.0, 0.0, 1.0], [item.pos[0] as f64, item.pos[1] as f64, UNIT as f64, UNIT as f64], c.transform, g);
            }

            for body in snake.bodies.iter() {
                rectangle([0.0, 1.0, 0.0, 1.0], [body.pos[0] as f64, body.pos[1] as f64, UNIT as f64, UNIT as f64], c.transform, g);
            }
        });
    }
}
