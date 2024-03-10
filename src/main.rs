//use std::f32::consts::PI;

use cli_clipboard;
use pathfinding::directed::astar::astar;
use rand::{self, random};
use speedy2d::color::Color;
use speedy2d::dimen::Vec2;
use speedy2d::shape::Rect;
use speedy2d::window::{MouseButton, VirtualKeyCode, WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

fn main() {
    let window = Window::new_centered("Speedy2D: Hello World", (640, 240)).unwrap();
    println!("Doing things");

    window.run_loop(MyWindowHandler::default());
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::BLACK);
        let size = helper.get_size_pixels();
        let x_size: f32 = size.x as f32 / self.x as f32;
        let y_size: f32 = size.y as f32 / self.y as f32;

        for x_cell in 0..self.x {
            for y_cell in 0..self.y {
                let top = y_size * y_cell as f32 + 1.0;
                let bott = y_size * y_cell as f32 + y_size - 1.0;
                let left = x_size * x_cell as f32 + 1.0;
                let right = x_size * x_cell as f32 + x_size - 1.0;
                let color: Color = match self.field[y_cell as usize][x_cell as usize] {
                    CellState::Empty => Color::from_rgb(1.0, 1.0, 1.0),
                    CellState::User => Color::from_rgb(0.2, 1.0, 0.3),
                    CellState::Tile => Color::from_rgb(0.2, 0.2, 0.3),
                    CellState::Enemy => Color::from_rgb(1.0, 0.2, 0.3),
                };
                graphics.draw_rectangle(
                    Rect::new(Vec2::new(left, top), Vec2::new(right, bott)),
                    color,
                )
            }
        }
        if let Some(v) = &self.path {
            for point in v {
                let pos = self.idx_to_px(helper, *point);
                graphics.draw_circle(pos, 4.0, Color::BLUE);
            }
        }
        helper.request_redraw();
    }
    fn on_mouse_button_down(&mut self, helper: &mut WindowHelper, button: MouseButton) {
        match button {
            MouseButton::Left => {
                let player = Coord {
                    x: self.player_pos[1],
                    y: self.player_pos[0],
                };
                let target = self.px_to_idx(helper, self.mouse_pos);
                let res = astar(
                    &player,
                    |c| c.successors(&self.field),
                    |c| c.distance(target),
                    |c| *c == target,
                );
                match res {
                    Some(v) => {
                        self.path = Some(v.0);
                        let v = self.path.as_ref().unwrap();
                        let cmd = Command::new(v.to_vec());
                        println!("{}", cmd.res);

                        cli_clipboard::set_contents(cmd.res).unwrap();
                    }
                    None => self.path = None,
                }
            }
            _ => (),
        }
    }
    fn on_mouse_move(&mut self, helper: &mut WindowHelper<()>, position: Vec2) {
        self.mouse_pos = position;
    }
    fn on_key_down(
        &mut self,
        helper: &mut WindowHelper<()>,
        virtual_key_code: Option<VirtualKeyCode>,
        scancode: speedy2d::window::KeyScancode,
    ) {
        if let Some(key) = virtual_key_code {
            let pos = self.px_to_idx(helper, self.mouse_pos);
            let idx_x = pos.x;
            let idx_y = pos.y;
            match key {
                VirtualKeyCode::T => {
                    if self.field[idx_y][idx_x] != CellState::Tile {
                        self.field[idx_y][idx_x] = CellState::Tile;
                    } else {
                        self.field[idx_y][idx_x] = CellState::Empty;
                    }
                }
                VirtualKeyCode::E => {
                    if self.field[idx_y][idx_x] != CellState::Enemy {
                        self.field[idx_y][idx_x] = CellState::Enemy;
                    } else {
                        self.field[idx_y][idx_x] = CellState::Empty;
                    }
                }
                VirtualKeyCode::U => {
                    if self.field[idx_y][idx_x] != CellState::User {
                        self.field[self.player_pos[0]][self.player_pos[1]] = CellState::Empty;
                        self.player_pos[0] = idx_y;
                        self.player_pos[1] = idx_x;
                        self.field[idx_y][idx_x] = CellState::User;
                    }
                }
                _ => (),
            }
        }
    }
}

struct Command {
    res: String,
}
impl Command {
    fn new(v: Vec<Coord>) -> Self {
        let mut ret = Command {
            res: "!".to_owned(),
        };
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        enum Direction {
            Left = 'l' as isize,
            Right = 'r' as isize,
            Up = 'u' as isize,
            Down = 'd' as isize,
        }
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        struct Subcommand {
            dir: Option<Direction>,
            count: u32,
        }

        fn subcmd_to_str(cmd: Subcommand) -> String {
            if cmd.dir == None || cmd.count == 0 {
                return String::new();
            }
            let mut ret = String::new();
            for _ in 0..cmd.count / 9 {
                ret.push(char::from_u32(cmd.dir.unwrap() as u32).unwrap());
                ret.push('9');
            }
            ret.push(char::from_u32(cmd.dir.unwrap() as u32).unwrap());
            ret += &((cmd.count % 9).to_string());
            ret
        }
        let mut prev = v[0];
        let mut sub = Subcommand {
            dir: None,
            count: 0,
        };
        for c in v.into_iter().skip(1) {
            if c.x != prev.x {
                if c.x > prev.x {
                    if sub.dir == Some(Direction::Right) {
                        sub.count += 1;
                    } else {
                        ret.res += &subcmd_to_str(sub);
                        sub.dir = Some(Direction::Right);
                        sub.count = 1;
                    }
                }
                if c.x < prev.x {
                    if sub.dir == Some(Direction::Left) {
                        sub.count += 1;
                    } else {
                        ret.res += &subcmd_to_str(sub);
                        sub.dir = Some(Direction::Left);
                        sub.count = 1;
                    }
                }
            }
            if c.y != prev.y {
                if c.y > prev.y {
                    if sub.dir == Some(Direction::Down) {
                        sub.count += 1;
                    } else {
                        ret.res += &subcmd_to_str(sub);
                        sub.dir = Some(Direction::Down);
                        sub.count = 1;
                    }
                }
                if c.y < prev.y {
                    if sub.dir == Some(Direction::Up) {
                        sub.count += 1;
                    } else {
                        ret.res += &subcmd_to_str(sub);
                        sub.dir = Some(Direction::Up);
                        sub.count = 1;
                    }
                }
            }
            prev = c;
        }
        ret.res += &subcmd_to_str(sub);
        ret.res += "a9a9a9a9a9";
        let c: u32 = random();
        ret.res += &c.to_string();
        ret
    }
}

struct MyWindowHandler {
    x: u32,
    y: u32,
    player_pos: [usize; 2],
    field: Vec<Vec<CellState>>,
    mouse_pos: Vec2,
    path: Option<Vec<Coord>>,
}

#[derive(PartialEq)]
enum CellState {
    Enemy,
    Empty,
    User,
    Tile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

impl MyWindowHandler {
    fn px_to_idx(&self, helper: &mut WindowHelper, pos: Vec2) -> Coord {
        let size = helper.get_size_pixels();
        let idx_x = ((pos.x / size.x as f32) * self.x as f32) as usize;
        let idx_x = idx_x.min(self.x as usize - 1);
        let idx_y = ((pos.y / size.y as f32) * self.y as f32) as usize;
        let idx_y = idx_y.min(self.y as usize - 1);
        Coord { x: idx_x, y: idx_y }
    }
    fn idx_to_px(&self, helper: &mut WindowHelper, pos: Coord) -> (f32, f32) {
        let size = helper.get_size_pixels();
        let mut px_x = (pos.x as f32 * (size.x as f32 / self.x as f32)) as f32;
        px_x += ((size.x / self.x) / 2) as f32;
        let mut px_y = (pos.y as f32 * (size.y as f32 / self.y as f32)) as f32;
        px_y += ((size.y / self.y) / 2) as f32;
        (px_x, px_y)
    }
}

impl Coord {
    fn successors(&self, field: &Vec<Vec<CellState>>) -> Vec<(Coord, u32)> {
        let height = field.len();
        let widht = field[0].len();
        let mut ret = Vec::new();
        if self.x > 0 && field[self.y][self.x - 1] == CellState::Empty {
            ret.push((
                Coord {
                    x: self.x - 1,
                    y: self.y,
                },
                1,
            ));
        }
        if self.x < widht - 1 && field[self.y][self.x + 1] == CellState::Empty {
            ret.push((
                Coord {
                    x: self.x + 1,
                    y: self.y,
                },
                1,
            ));
        }
        if self.y < height - 1 && field[self.y + 1][self.x] == CellState::Empty {
            ret.push((
                Coord {
                    x: self.x,
                    y: self.y + 1,
                },
                1,
            ));
        }
        if self.y > 0 && field[self.y - 1][self.x] == CellState::Empty {
            ret.push((
                Coord {
                    x: self.x,
                    y: self.y - 1,
                },
                1,
            ));
        }
        ret
    }
    fn distance(&self, p2: Self) -> u32 {
        (self.x.abs_diff(p2.x) + self.y.abs_diff(p2.y)) as u32
    }
}

impl Default for MyWindowHandler {
    fn default() -> Self {
        let mut field: Vec<Vec<_>> = Vec::new();
        for y in 0..13 {
            field.push(Vec::<CellState>::new());
            for _ in 0..19 {
                field[y].push(CellState::Empty);
            }
        }
        field[0][0] = CellState::User;
        MyWindowHandler {
            x: 19,
            y: 13,
            player_pos: [0, 0],
            field,
            mouse_pos: Vec2::new(0.0, 0.0),
            path: None,
        }
    }
}
