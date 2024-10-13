use std::ffi::{CStr, CString};

macro_rules! max {
    ( $a:expr, $b:expr ) => {
        (if $a > $b { $a } else {$b})
    };
}
macro_rules! min {
    ( $a:expr, $b:expr ) => {
        (if $a < $b { $a } else {$b})
    };
}

macro_rules! abs {
    ($a:expr) => {
        (if $a < 0 {-a} else {a})
    };
}

macro_rules! in_range {
    (($a:expr, $b:expr), $c:expr) => {
        ($a <= $c && $b >= $c)
    };
}

macro_rules! clamp {
    (($a:expr, $b:expr), $c:expr) => {
        max!($a, min!($b, $c))
    };
}

const WIDTH: u16 = 800;
const HEIGHT: u16 = 600;
const BALL_SIZE: u8 = 10;
const PADDLE_LENGTH: u16 = 120;
const PADDLE_WIDTH: u8 = 10;
const  MAX_SCORE: u8 = 5;

const DEFAULT_BALL_X: f32 =  (WIDTH as f32) * 0.5f32;
const DEFAULT_BALL_Y: f32 = (HEIGHT as f32) * 0.5f32;
const DEFAULT_BALL_X_SPEED: f32 =  80.0;
const DEFAULT_BALL_Y_SPEED: f32 =  0.0;
const BALL_BOUNCE_SPEED_MULTIPLIER: f32 = -1.2;
const PADDLE_BOUNCE_SPEED_MULTIPLIER: f32 = 0.8;

const PLAYER_ACCEL: f32 = 300.0;
const PLAYER_MAX_SPEED: f32 = 200.0;
const PLAYER_DECEL: f32 = 600.0;

struct Ball {
    x: f32,
    y: f32,
    x_speed: f32,
    y_speed: f32
}

struct PlayerStruct {
    position: f32,
    speed: f32,
    score: u8
}

impl PlayerStruct {
    fn handle_input(self: &mut PlayerStruct, up: bool, down: bool, delta_time_sec: f32) {
        let input = (down as i8) - (up as i8);
        if input != 0 {
            self.speed += PLAYER_ACCEL * (input as f32) * delta_time_sec;
            self.speed = clamp!((-PLAYER_MAX_SPEED, PLAYER_MAX_SPEED), self.speed);
        }
        else {
            if self.speed != 0.0{
                let dir = self.speed.signum();
                self.speed -= dir * PLAYER_DECEL * delta_time_sec;
                if (self.speed.signum() as i8) != (dir as i8) {
                    self.speed = 0.0;
                }
            }
        }
        self.position += self.speed * delta_time_sec;
        self.position = clamp!((0f32, (HEIGHT - PADDLE_LENGTH) as f32), self.position);

    }
    
}

enum Player {
    P1(PlayerStruct),
    P2(PlayerStruct)
}


impl Ball {
    fn touching_player(self: &mut Ball, player: &Player) -> bool {
        let p: &PlayerStruct = match player {
            Player::P1(p) => {&p},
            Player::P2(p) => {&p}
        };
        let clamped_ball_pos: u32 = clamp!((0, (HEIGHT - 1) as u32), self.y as u32);

        let is_touching = in_range!((p.position as u32, p.position as u32 + (PADDLE_LENGTH as u32)), clamped_ball_pos);
        if is_touching {
            self.y_speed += p.speed * PADDLE_BOUNCE_SPEED_MULTIPLIER;
        }
        return is_touching;
    }

    fn calc_ball_physics_cycle(self: &mut Ball, delta_time_sec: f32) -> bool{
        self.x += self.x_speed * delta_time_sec;
        self.y += self.y_speed * delta_time_sec;
        let mut x_bounce = false;
        
        if !in_range!((0, (WIDTH - 1) as i32), self.x as i32) {
            self.x = clamp!((0.0, (WIDTH - 1) as f32), self.x);
            self.x_speed *= BALL_BOUNCE_SPEED_MULTIPLIER;
            x_bounce = true;
        }

        if !in_range!((0, (HEIGHT - 1) as i32), self.y as i32) {
            self.y = clamp!((0.0, (HEIGHT - 1) as f32), self.y);
            self.y_speed *= -1.0;
        }
        return x_bounce;
    }

    fn reset(self: &mut Ball) {
        self.x = DEFAULT_BALL_X;
        self.y = DEFAULT_BALL_Y;
        self.x_speed = DEFAULT_BALL_X_SPEED;
        self.y_speed = DEFAULT_BALL_Y_SPEED;
    }
}


extern "C" {
    fn init(width: i32, height: i32,  paddle_length: u16, paddle_width: u8, ball_size: u8);
    fn get_time_milis() -> u32;
    fn draw(ball_x: f32, ball_y: f32, p1_height: f32, p2_height: f32);
    fn update_SDL();
    fn finish_game();
    fn set_title(title: *const u8);
    static mut p1_up: bool;
    static mut p1_down: bool;
    static mut p2_up: bool;
    static mut p2_down: bool;
    static mut game_over: bool;
}

fn main() {
    let mut p1 = Player::P1(PlayerStruct{position: 0f32, speed: 0f32, score: 0});
    let mut p2 = Player::P2(PlayerStruct{position: 0f32, speed: 0f32, score: 0});
    let mut ball = Ball{
        x: DEFAULT_BALL_X,
        y: DEFAULT_BALL_Y,
        x_speed: DEFAULT_BALL_X_SPEED,
        y_speed: DEFAULT_BALL_Y_SPEED
    };
    let mut start_time_milis;
    unsafe {
        init(WIDTH as i32, HEIGHT as i32, PADDLE_LENGTH, PADDLE_WIDTH, BALL_SIZE);
        start_time_milis = get_time_milis();
    }

    loop {
        let cur_time_milis;
        unsafe {
            cur_time_milis = get_time_milis();
        }

        let delta_time_seconds = ((cur_time_milis - start_time_milis) as f32) * 0.001;
        start_time_milis = cur_time_milis;
        let p1_height: f32;
        let p2_height: f32;
        let p1_score: u8;
        let p2_score: u8;

        match p1 {
            Player::P1(ref mut p) => {
                if p.score > MAX_SCORE {
                    break;
                }
                unsafe {
                    p.handle_input(p1_up, p1_down, delta_time_seconds);
                }
                p1_height = p.position;
                p1_score = p.score;
            },
            _ => {panic!("p1 was dropped!")}
        };
        match p2 {
            Player::P2(ref mut p) => {
                if p.score > MAX_SCORE {
                    break;
                }
                unsafe {
                    p.handle_input(p2_up, p2_down, delta_time_seconds);
                }
                p2_height = p.position;
                p2_score = p.score;
            },
            _ => {panic!("p2 was dropped!")}
        };

        if ball.calc_ball_physics_cycle(delta_time_seconds) {
            if ball.x < 0.5*(WIDTH as f32){
                // p2 scoring
                if !ball.touching_player(&p1) {
                    match p2 {
                        Player::P2(ref mut p) => {
                            p.score += 1;
                        },
                        _ => {panic!("p2 was dropped!")}
                    };
                    ball.reset();
                }
            }
            else if !ball.touching_player(&p2){
                // p1 scoring
                match p1 {
                    Player::P1(ref mut p) => {
                        p.score += 1;
                    },
                    _ => {panic!("p1 was dropped!")}
                };
                ball.reset();
            }
        }

        unsafe {
            set_title(CString::new(format!("{p1_score}  :  {p2_score}").as_str()).unwrap().as_bytes_with_nul().as_ptr());
            draw(ball.x, ball.y, p1_height, p2_height);
            update_SDL();
            if game_over {
                break;
            }
        }
    }
    unsafe  {
        finish_game();
    }
}
