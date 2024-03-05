#![allow(dead_code)]

mod snake;

use std::{
    cell, io::{self, Write}, thread, vec
};

use snake::Snake;

// "C:\Windows\System32\downlevel\api-ms-win-crt-conio-l1-1-0.dll"
// C:\Program Files (x86)\Windows Kits\10\Include\10.0.19041.0\ucrt\conio.h
// https://docs.rs/conio_rs/latest/x86_64-pc-windows-msvc/src/conio_rs/lib.rs.html
// Link api-ms-win-crt-conio-l1-1-0.dll
#[link(name = "api-ms-win-crt-conio-l1-1-0", kind = "raw-dylib")]
extern "C" {
    fn _getch() -> i32;
    fn _kbhit() -> bool;
}

// Implementation

/// Enter 1 byte from the console.
pub fn getch() -> i32 {
    unsafe { _getch() }
}

/// _kbhit
pub fn kbhit() -> bool {
    unsafe { _kbhit() }
}

fn main() {
    let mut board = vec![vec![' '; 20]; 20];
    let mut snake = Snake::new();
    let mut ch: char = 'a';

    loop {
        if kbhit() {
            ch = getch() as u8 as char;
            match ch {
                'w' => snake.up(),
                'a' => snake.left(),
                's' => snake.down(),
                'd' => snake.right(),
                'q' => return,
                _ => {}
            }
            // let _ = io::stdout().write(&[ch]);
            // let _ = io::stdout().flush();
        }
        snake.tick();
        snake.draw(&mut board);

        println!("----------{}----------", ch);
        board.iter().for_each(|row| {
            row.iter().for_each(|cell| print!("{}", cell));
            print!("\n");
        });
        println!("--------------------");

        let duration = std::time::Duration::from_millis(200);
        thread::sleep(duration);
    }
}
