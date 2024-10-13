// macro_rules! max {
//     ( $a:expr, $b:expr ) => {
//         if $a > $b { $a } else {$b}
//     };
// }
// macro_rules! min {
//     ( $a:expr, $b:expr ) => {
//         if $a < $b { $a } else {$b}
//     };
// }

// macro_rules! abs {
//     ($a:expr) => {
//         if $a < 0 {-a} else {a}
        
//     };
// }

// macro_rules! in_range {
//     (($a:expr, $b:expr), $c:expr) => {
//         ($a <= $c && $b >= $c)
//     };
// }

// macro_rules! clamp {
//     (($a:expr, $b:expr), $c:expr) => {
//         max!($a, min!($b, $c))
//     };
// }

// const WIDTH: u32 = 800;
// const HEIGHT: u32 = 600;
// const PADDLE_LENGTH: u32 = 100;

// struct Ball {
//     x: f32,
//     y: f32,
//     x_speed: f32,
//     y_speed: f32
// }

// struct PlayerStruct {
//     position: f32,
//     speed: f32,
//     score: u8
// }

// enum Player {
//     P1(PlayerStruct),
//     P2(PlayerStruct)
// }


// impl Ball {
//     fn touching_player(self: &Ball, player: &Player) -> bool {
//         let paddle_x: u32;
//         let p: &PlayerStruct = match player {
//             Player::P1(p) => {paddle_x = 0; &p},
//             Player::P2(p) => {paddle_x = WIDTH - 1; &p}
//         };
//         let clamped_ball_pos: u32 = clamp!((0, (WIDTH - 1) as u32), self.x as u32);

//         in_range!((p.position as u32, p.position as u32 + PADDLE_LENGTH), clamped_ball_pos) && paddle_x == clamped_ball_pos
//     }

//     fn calc_ball_physics_cycle(self: &mut Ball, delta_time_sec: f32) {
//         self.x += self.x_speed * delta_time_sec;
//         self.y += self.y_speed * delta_time_sec;
        
//         if !in_range!((0, WIDTH - 1), self.x as u32) {
//             self.x = clamp!((0.0, (WIDTH - 1) as f32), self.x);
//             self.x_speed *= -1.0;
//         }

//         if !in_range!((0, HEIGHT - 1), self.y as u32) {
//             self.y = clamp!((0.0, (HEIGHT - 1) as f32), self.y);
//             self.y_speed *= -1.0;
//         }
//     }
// }


extern "C" {
    fn init(width: i32, height: i32,  paddle_length: u16, paddle_width: u8, ball_size: u8);
    fn get_time_milis() -> u32;
    fn draw(ball_x: f32, ball_y: f32, p1_height: f32, p2_height: f32);
    fn update_SDL();
    static mut p1_up: bool;
    static mut p1_down: bool;
    static mut p2_up: bool;
    static mut p2_down: bool;
    static mut game_over: bool;
}

fn main() {
    loop {
        unsafe {
            init(100, 100, 30, 5, 5);
            draw(50.0, 50.0, 0.0, 0.0);
            update_SDL();
        }
    }
    // let p1 = Player::P1(PlayerStruct{position: 0f32, speed: 0f32, score: 0});
    // let p2 = Player::P2(PlayerStruct{position: 0f32, speed: 0f32, score: 0});
}
