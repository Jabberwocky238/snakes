#![allow(dead_code)]

mod snake;
mod board;
use snake::Snake;
use board::Board;

use std::{
    cell, io::{self, Write}, thread, vec
};


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
    let mut board = Board::new();
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
        
        println!("----------{}----------", ch);
        snake.try_eat();
        board.draw(&mut snake);
        snake.tick();

        let duration = std::time::Duration::from_millis(200);
        thread::sleep(duration);
    }
}
