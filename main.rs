// A Simple Terminal Example Of PIDs In Rust
// Author: Jackson McDowell
use rand::Rng;
use std::process::Command;
use device_query::{DeviceQuery, DeviceState, Keycode};

const WIDTH: i8 = 40;
const HEIGHT: i8 = 40;


#[derive(Copy, Clone)]
struct PID {
    proportional: f32,
    integral: f32,
    derivative: f32,
    x: i8,
    y: i8,
    speed: f32,
    angle: u16,
    distance: f32,
    is_alive: bool,
    is_player: bool,
}

#[derive(Copy, Clone)]
struct Player {
    x: i8,
    y: i8,
}

fn print_board(player: Player, pid1: PID, pid2: PID, pid3: PID) {
    // Clear the screen
    print!("\x1B[2J\x1B[1;1H");
    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            if j == player.x && i == player.y {
                print!("\x1b[93mX\x1b[0m ");
            } else if j == pid1.x && i == pid1.y {
                print!("\x1b[91m0\x1b[0m ");
            } else if j == pid2.x && i == pid2.y {
                print!("\x1b[92m0\x1b[0m ");
            } else if j == pid3.x && i == pid3.y {
                print!("\x1b[94m0\x1b[0m ");
            } else {
                print!("- ");
            }
        }
        println!();
    }
}

fn main() {

    // Player
    let mut player = Player {
        x: WIDTH / 2,
        y: HEIGHT / 2,
    };

    // PID 1
    let mut pid1 = PID {
        proportional: 0.0,
        integral: 0.0,
        derivative: 0.0,
        x: 0,
        y: 0,
        speed: 0.0,
        angle: 0,
        distance: 0.0,
        is_alive: true,
        is_player: false,
    };

    // PID 2
    let mut pid2 = PID {
        proportional: 0.0,
        integral: 0.0,
        derivative: 0.0,
        x: 0,
        y: 0,
        speed: 0.0,
        angle: 0,
        distance: 0.0,
        is_alive: true,
        is_player: false,
    };

    // PID 3
    let mut pid3 = PID {
        proportional: 0.0,
        integral: 0.0,
        derivative: 0.0,
        x: 0,
        y: 0,
        speed: 0.0,
        angle: 0,
        distance: 0.0,
        is_alive: true,
        is_player: false,
    };

    // Generate Random PID Locations
    let mut rng = rand::thread_rng();
    pid1.x = rng.gen_range(0..WIDTH);
    pid1.y = rng.gen_range(0..HEIGHT);

    pid2.x = rng.gen_range(0..WIDTH);
    pid2.y = rng.gen_range(0..HEIGHT);

    pid3.x = rng.gen_range(0..WIDTH);
    pid3.y = rng.gen_range(0..HEIGHT);

    // Keyboard Input
    let device_state = DeviceState::new();

    // Main Loop
    loop {
        print_board(player, pid1, pid2, pid3);

        // Get Keyboard Input
        let keys: Vec<Keycode> = device_state.get_keys();
        for key in keys.iter() {
            match key {
                Keycode::W => {
                    if player.y > 0 {
                        player.y -= 1;
                    }
                }
                Keycode::A => {
                    if player.x > 0 {
                        player.x -= 1;
                    }
                }
                Keycode::S => {
                    if player.y < HEIGHT - 1 {
                        player.y += 1;
                    }
                }
                Keycode::D => {
                    if player.x < WIDTH - 1 {
                        player.x += 1;
                    }
                }
                _ => {}
            }
        }







        let mut child = Command::new("sleep").arg("0.11").spawn().unwrap();
        let _result = child.wait().unwrap();
    }
    


}
