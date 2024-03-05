#![allow(dead_code)]

mod snake;

use std::{io::{self, Write}, thread};

// "C:\Windows\System32\downlevel\api-ms-win-crt-conio-l1-1-0.dll"
// https://docs.rs/conio_rs/latest/x86_64-pc-windows-msvc/src/conio_rs/lib.rs.html
// Link api-ms-win-crt-conio-l1-1-0.dll
#[link(name="api-ms-win-crt-conio-l1-1-0", kind="raw-dylib")]
extern{
    fn _getch() -> i32;
    fn _kbhit() -> bool;
}

// Implementation

/// Enter 1 byte from the console.
pub fn getch() -> i32 {
    unsafe {
        _getch()
    }
}

/// _kbhit
pub fn kbhit() -> bool {
    unsafe {
        _kbhit()
    }
}

fn main(){
    let mut ch: u8 = b' ';
	loop
	{
        if kbhit() {
            ch = getch() as u8;
            let _ = io::stdout().write(&[ch]);
            let _ = io::stdout().flush();
        }
        else {
            let _ = io::stdout().write(b"none\n");
            let _ = io::stdout().flush();
        }
        let duration = std::time::Duration::from_millis(100);
        thread::sleep(duration);
	}
}