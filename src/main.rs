macro_rules! max {
    ( $a:expr, $b:expr ) => {
        if $a > $b { $a } else {$b}
    };
}
macro_rules! min {
    ( $a:expr, $b:expr ) => {
        if $a < $b { $a } else {$b}
    };
}

macro_rules! abs {
    ($a:expr) => {
        if $a < 0 {-a} else {a}
        
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

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const PADDLE_LENGTH: usize = 100;
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

enum Player {
    P1(PlayerStruct),
    P2(PlayerStruct)
}


impl Ball {
    fn touching_player(self: &Ball, player: &Player) -> bool {
        let paddle_x: usize;
        let p: &PlayerStruct = match player {
            Player::P1(p) => {paddle_x = 0; &p},
            Player::P2(p) => {paddle_x = WIDTH - 1; &p}
        };
        let clamped_ball_pos: usize = clamp!((0, (WIDTH - 1) as usize), self.x as usize);

        in_range!((p.position as usize, p.position as usize + PADDLE_LENGTH), clamped_ball_pos) && paddle_x == clamped_ball_pos
    }

    fn calc_ball_physics_cycle(self: &mut Ball, delta_time_sec: f32) {
        self.x += self.x_speed * delta_time_sec;
        self.y += self.y_speed * delta_time_sec;
        
        if !in_range!((0, WIDTH - 1), self.x as usize) {
            self.x = clamp!((0.0, (WIDTH - 1) as f32), self.x);
            self.x_speed *= -1.0;
        }

        if !in_range!((0, HEIGHT - 1), self.y as usize) {
            self.y = clamp!((0.0, (HEIGHT - 1) as f32), self.y);
            self.y_speed *= -1.0;
        }
    }
}

fn main() {
    println!("Hello, world!");
}
