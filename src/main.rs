mod game;

use std::{io::{self, Read}, process::exit, thread::{self, sleep}, time::Duration};

use game::{point, GameStatus};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver};
use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};


fn main() {
    let mut game = GameStatus::new(10, 10);
    for _ in 0..10 {
        game.generate_food();
    }

    let mut direction = point(1, 0);
    let (read_channel, termios) = spawn_stdin_channel();

    loop {
        print!("\x1B[2J\x1B[1;1H");
        match read_channel.try_recv() {
            Ok(key) => {
                let new_direction = match std::str::from_utf8(&key) {
                    Ok("d") => point(0, 1),
                    Ok("a") => point(0, -1),
                    Ok("s") => point(1, 0),
                    Ok("w") => point(-1, 0),
                    _ => direction,
                };
                if new_direction + direction != point (0, 0) {
                    direction = new_direction;
                }
            }
            Err(mpsc::TryRecvError::Empty) => (),
            Err(mpsc::TryRecvError::Disconnected) => (),
        }

        game.generate_food_if_empty();

        if !game.snake.move_snake(direction) || game.is_gameover() {
            tcsetattr(0, TCSANOW, & termios).unwrap();
            exit(1);
        }

        game.eat();

        game.update_matrix();

        // Print status
        for i in 0..game.width {
            for j in 0..game.height {
                print!("{}", &game.get_cell(point(i as i32, j as i32)).unwrap())
            }
            println!("");
        }
        print!("\n");
        sleep(Duration::new(0, 2 * 1e8 as u32));
    }
}

fn spawn_stdin_channel() -> (Receiver<Vec<u8>>, Termios) {
    let (tx, rx) = mpsc::channel::<Vec<u8>>();
    let stdin = 0; // couldn't get std::os::unix::io::FromRawFd to work 
                   // on /dev/stdin or /dev/tty
    let termios = Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone();  // make a mutable copy of termios 
                                            // that we will modify
    new_termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode
    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
    let mut reader = io::stdin();

    thread::spawn(move || loop {
        let mut buffer = [0;1];  // read exactly one byte

        if let Ok(_) = reader.read_exact(&mut buffer) {
            if let Err(e) = tx.send(buffer.into()) {
                println!("{}", e);
            }
        }
    });
    (rx, termios)
}
