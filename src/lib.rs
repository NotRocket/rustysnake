use speedy2d::Window;
use speedy2d::window::{
    WindowHandler,
    WindowHelper,
    KeyScancode,
    VirtualKeyCode,
    WindowStartupInfo,
    UserEventSender,
};
use speedy2d::{Graphics2D};
use speedy2d::color::Color;
use std::time::{Duration, Instant};
use std::collections::VecDeque;
use rand::prelude::*;

#[derive(Copy, Clone,PartialEq)]
enum TileState{
    SnakeOccupied,
    FoodOccupied,
    BorderDeathZone,
    Free
}
#[derive(Copy, Clone)]
struct Tile{
    state: TileState,
    x_coordinate: usize,
    y_coordinate: usize,
}
struct GameBoard{
    board: Vec<Vec<Tile>>,
    length: usize,
    width: usize,
}
impl GameBoard{
    fn new(length: usize, width: usize) -> GameBoard{
        let buildaboard: Vec<Vec<Tile>> = (0..width).map(|y| {
            (0..length).map(|x| {
                Tile {
                    state: TileState::Free,
                    x_coordinate: x,
                    y_coordinate: y,
                }
            }).collect()
        }).collect();

        let mut _board = GameBoard{
            board: buildaboard,
            length,
            width,
        };
        _board.generate_borders();
        _board.board[_board.length / 2][_board.width / 2].state = TileState::SnakeOccupied;
        _board.spawn_food();

        _board
    }

    fn generate_borders(&mut self){
        // top row
        for tile in self.board[0].iter_mut(){
            tile.state = TileState::BorderDeathZone;
        }
        // bottom row
        for tile in self.board[self.length-1].iter_mut(){
            tile.state = TileState::BorderDeathZone;
        }
        for edge in 0..self.width-1{
            self.board[edge][0].state = TileState::BorderDeathZone; // left edge
            self.board[edge][self.width -1].state = TileState::BorderDeathZone; // right edge
        }
    }

    fn spawn_food(&mut self){
        let mut freepool: Vec<Tile> = Vec::new();
        for y in &self.board{
            for x in y{
                if x.state == TileState::Free{
                    freepool.push(x.clone());
                }
            }
        }

        let mut rng = thread_rng();
        let mut selectedTile = freepool.choose(&mut rng).unwrap();
        self.board[selectedTile.y_coordinate][selectedTile.x_coordinate].state = TileState::FoodOccupied;
    }
}

struct Snake{
    head: Tile,
    tail: Tile,
    body: VecDeque<Tile>,
    maxlength: usize,
    alive: bool,
}

impl Snake{
    fn new(mut board: &mut GameBoard) -> Snake{
        let mut head = board.board[board.length / 2][board.width / 2];
        head.state = TileState::SnakeOccupied;
        board.board[head.y_coordinate][head.x_coordinate].state = TileState::SnakeOccupied;
        let mut body = VecDeque::from(vec![head]);
        Snake{
            head: head,
            tail: head,
            body: body,
            maxlength: 1,
            alive: true,
        }
    }
    fn move_snake(&mut self, board: &mut GameBoard, key: VirtualKeyCode){
        match key{
            VirtualKeyCode::W => {
                self.head = board.board[self.head.y_coordinate - 1][self.head.x_coordinate];
            },
            VirtualKeyCode::S => {
                self.head = board.board[self.head.y_coordinate + 1][self.head.x_coordinate];
            },
            VirtualKeyCode::A => {
                self.head = board.board[self.head.y_coordinate][self.head.x_coordinate - 1];
            },
            VirtualKeyCode::D => {
                self.head = board.board[self.head.y_coordinate][self.head.x_coordinate + 1];
            },
            _ => {}
        }
        match board.board[self.head.y_coordinate][self.head.x_coordinate].state{
            TileState::FoodOccupied => {
                board.board[self.head.y_coordinate][self.head.x_coordinate].state = TileState::SnakeOccupied;
                self.body.push_back(self.head);
                self.maxlength += 1;
                board.spawn_food();

            },
            TileState::BorderDeathZone => {
                self.alive = false;
            },
            TileState::SnakeOccupied => {
                self.alive = false;
            }
            TileState::Free => {
                board.board[self.head.y_coordinate][self.head.x_coordinate].state = TileState::SnakeOccupied;
                self.body.push_back(self.head);
            }
        }
        if self.body.len() > self.maxlength{
            let mut removed_tile = self.body.pop_front().unwrap();
            board.board[removed_tile.y_coordinate][removed_tile.x_coordinate].state = TileState::Free;
        }

    }
}
struct MyWindowHandler {
    last_key: Option<VirtualKeyCode>,
    last_check: Instant,
    game_board: Option<GameBoard>,
    snake: Option<Snake>,
}

impl MyWindowHandler{
    fn new() -> Self{
        let mut wh = MyWindowHandler{last_key: None, last_check: Instant::now(),game_board: None,snake: None};
        wh.game_board = Option::from(GameBoard::new(25, 25));
        wh.snake = Option::from(Snake::new(wh.game_board.as_mut().unwrap()));
        wh
    }
    fn reset(&mut self){
        self.game_board = Option::from(GameBoard::new(25, 25));
        self.snake = Option::from(Snake::new(self.game_board.as_mut().unwrap()));
    }
}

impl WindowHandler for MyWindowHandler
{
    fn on_start(&mut self, helper: &mut WindowHelper, info: WindowStartupInfo) {
        helper.set_title("Rusty Snake");
        //self.game_board = Option::from(GameBoard::new(25, 25));
        // self.snake = Option::from(Snake::new(self.game_board.as_mut().unwrap()));
    }

    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));
        let mut x: f32 = 50.0;
        let mut y: f32 = 50.0;
        for tileVector in self.game_board.as_mut().unwrap().board.iter_mut() {
            for tile in tileVector.iter_mut() {
                graphics.draw_rectangle(
                    speedy2d::shape::Rectangle::from_tuples((x, y), (x + 49.0, y + 49.0)),
                    match tile.state {
                        TileState::FoodOccupied => { Color::RED }
                        TileState::BorderDeathZone => { Color::BLACK }
                        TileState::Free => { Color::WHITE }
                        TileState::SnakeOccupied => { if self.snake.as_ref().unwrap().alive {
                            Color::BLUE
                        }else{
                            Color::GREEN
                        }
                        }
                    },
                );
                x += 50.0;
            }
            x = 50.0;
            y += 50.0;
        }
        if self.snake.as_ref().unwrap().alive{
            if self.last_check.elapsed() > Duration::from_secs_f64(0.25) {
                match self.last_key {
                    Some(key) => {self.snake.as_mut().unwrap().move_snake(self.game_board.as_mut().unwrap(), key); },
                    None => { println!("No key pressed") }
                }

                self.last_check = Instant::now();
            }
        }else if !self.snake.as_ref().unwrap().alive{
            match self.last_key {
                Some(key) =>{if key == VirtualKeyCode::Space{
                    self.reset();
                    self.last_key = None;
                }}
                None => {}
            }
        }

        helper.request_redraw();
    }
    fn on_key_down(&mut self, helper: &mut WindowHelper<()>, virtual_key_code: Option<VirtualKeyCode>, scancode: KeyScancode) {
        if let Some(key) = virtual_key_code {
            self.last_key = Some(key);
        }
    }
}

fn main() {
    let window = Window::<()>::new_centered("Title", (1350, 1350)).unwrap();
    window.run_loop(MyWindowHandler::new());
}


