use notan::draw::*;
use notan::prelude::*;
use rand::prelude::*;
use std::collections::HashMap;
use std::collections::VecDeque;

const TILEL: i32 = 20;
const SCREENW: i32 = 640;
const SCREENH: i32 = 480;
const COOLTIME: f64 = 0.5;
const COOLRATE: f64 = 0.1;

type Snake = VecDeque<(i32, i32)>;
type SnakeSet = HashMap<(i32, i32), i32>;
#[derive(AppState)]
pub enum GameState {
    Title,
    Play {
        snake: Snake,
        snake_points: SnakeSet,
        apple: (i32, i32),
        direction: Direction,
        cooldown: f64, //frame
    },
    Death,
}
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
    Empty,
}
pub fn h_setup() -> GameState {
    GameState::Title
}
fn h_setup_play() -> GameState {
    GameState::Play {
        snake: VecDeque::from([(1, 1)]),
        snake_points: SnakeSet::new(),
        apple: (2, 2),
        direction: Direction::Right,
        cooldown: COOLTIME,
    }
}

fn h_update_vdeque(snake: &mut Snake, direction: &Direction, snake_points: &mut SnakeSet) {
    if let Some((last_x, last_y)) = snake.front() {
        let owned_x = *last_x;
        let owned_y = *last_y;
        match direction {
            Direction::Up => snake.push_front((owned_x, last_y - 1)),
            Direction::Down => snake.push_front((owned_x, last_y + 1)),
            Direction::Left => snake.push_front((last_x - 1, owned_y)),
            Direction::Right => snake.push_front((last_x + 1, owned_y)),
            _ => {}
        }
    }
    if let Some(new_coords) = snake.front() {
        if let Some(old_count) = snake_points.get_mut(new_coords) {
            *old_count += 1
        } else {
            snake_points.insert(*(new_coords), 1);
        }
    }
}
fn h_update_direction(direction: &mut Direction, transaction: Direction) {
    if transaction == Direction::Empty{
        return
    }
    match direction {
        Direction::Up if transaction == Direction::Down => {}
        Direction::Down if transaction == Direction::Up => {}
        Direction::Left if transaction == Direction::Right => {}
        Direction::Right if transaction == Direction::Left => {}
        _ => *direction = transaction,
    }
}
pub fn h_update(app: &mut App, state: &mut GameState) {
    match state {
        GameState::Play {
            snake,
            snake_points,
            apple,
            direction,
            cooldown,
        } => {
            let mut attempt: Direction = Direction::Empty;
            if app.keyboard.was_pressed(KeyCode::W) {
                //shitty handling
                attempt = Direction::Up;
            }

            if app.keyboard.was_pressed(KeyCode::D) {
                attempt = Direction::Right;
            }

            if app.keyboard.was_pressed(KeyCode::A) {
                attempt = Direction::Left
            }

            if app.keyboard.was_pressed(KeyCode::S) {
                attempt = Direction::Down
            } //put this before because it doesnt really let you input
            h_update_direction(direction, attempt);
            if *cooldown > 0.0 {
                *cooldown -= COOLRATE;
                return;
            }

            h_update_vdeque(snake, direction, snake_points);
            if let Some(tail) = snake.pop_back() {
                if let Some(tail_decrease) = snake_points.get_mut(&tail) {
                    *tail_decrease -= 1
                }
            }

            if let Some((now_x, now_y)) = snake.front() {
                if now_x * TILEL > SCREENW
                    || now_x < &0i32
                    || now_y * TILEL > SCREENH
                    || now_y < &0i32
                {
                    *state = GameState::Death;
                    return;
                }
                let (a_x, a_y) = apple;
                if *now_x == *a_x && *now_y == *a_y {
                    //apple eatne, generate a new apple
                    let mut rng = thread_rng();

                    let up_bound_x = 31; //sorry about the magic number
                    let up_bound_y = 23;
                    let a_newx: i32 = rng.gen_range(0..up_bound_x);
                    let a_newy: i32 = rng.gen_range(0..up_bound_y);
                    *a_x = a_newx;
                    *a_y = a_newy;

                    //push a new  one in
                    h_update_vdeque(snake, direction, snake_points);
                }
            } //collision

            *cooldown = COOLTIME;
            for (_, value) in snake_points {
                if *value > 1 {
                    *state = GameState::Death;
                    break;
                }
            }
        }
        GameState::Death => {
            if app.keyboard.was_pressed(KeyCode::Space) {
                *state = h_setup_play();
            }
        }
        GameState::Title => {
            if app.keyboard.was_pressed(KeyCode::Space) {
                *state = h_setup_play();
            }
        }
    }
}

pub fn h_draw(app: &mut App, gfx: &mut Graphics, state: &mut GameState) {
    //tile size (square)
    let h_font = gfx
        .create_font(include_bytes!("assets/Ubuntu-B.ttf"))
        .unwrap();
    h_update(app, state);

    let mut drawer = gfx.create_draw();
    drawer.clear(Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    });

    match state {
        GameState::Play { snake, apple, .. } => {
            for (x, y) in snake {
                drawer.rect(
                    ((TILEL * *x) as f32, (&TILEL * *y) as f32),
                    (TILEL as f32, TILEL as f32),
                );
            }
            let (a_x, a_y) = apple;
            drawer
                .rect(
                    ((*a_x * TILEL) as f32, (*a_y * TILEL) as f32),
                    (TILEL as f32, TILEL as f32),
                )
                .fill_color(Color::RED);
        }
        GameState::Death => {
            //how to center a div:
            drawer.text(&h_font, "you dead").position(240.0, 240.0);
            drawer
                .text(&h_font, "press space to continue")
                .position(240.0, 280.0);
        }
        GameState::Title => {
            drawer
                .text(&h_font, "snek by \"blahbaka\" (c) 2023")
                .position(240.0, 240.0);
        }
    }
    gfx.render(&drawer);
}
