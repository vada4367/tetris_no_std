#![allow(invalid_value)]
#![no_std]
#![no_main]

// Import Shapes structs and Screen struct
mod shape;
use crate::shape::Shape;

mod screen;
use crate::screen::Screen;

// For init fd_set and termios var-s
use core::mem::MaybeUninit;

// For support all system (on android *const u8)
type Cstr = *const i8;

// Import libc types and a few funcs/macroses
extern crate libc;
use libc::{
    c_char, c_int, c_uint, c_void, fd_set, size_t, termios, time_t, timeval, ECHO, FD_ISSET,
    FD_SET, FD_ZERO, ICANON, STDIN_FILENO, TCSANOW,
};

// Link libc funcs
#[cfg(target_os = "linux")]
#[link(name = "c")]

extern "C" {
    fn free(p: *mut c_void);
    fn malloc(size: size_t) -> *mut c_void;
    fn time(time: *mut time_t) -> time_t;
    fn srand(seed: c_uint);
    fn printf(format: *const c_char, ...) -> c_int;
    fn usleep(secs: c_uint) -> c_int;
    fn tcgetattr(fd: c_int, termios: *mut termios) -> c_int;
    fn tcsetattr(fd: c_int, optional_actions: c_int, termios: *const termios) -> c_int;
    fn getchar() -> c_int;
    fn select(
        nfds: c_int,
        readfds: *mut fd_set,
        writefds: *mut fd_set,
        errorfds: *mut fd_set,
        timeout: *mut timeval,
    ) -> c_int;
    fn system(s: *const c_char) -> c_int;
}

// For Box<T> and Vec<T>
extern crate alloc;
use core::alloc::{GlobalAlloc, Layout};

#[derive(Default)]
pub struct Allocator;
unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        malloc(layout.size() as usize) as *mut u8
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr as *mut c_void);
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    // Init random
    unsafe {
        srand(time(core::ptr::null_mut()) as u32);
    }
    // Input init
    let mut oldt;
    let mut newt;
    unsafe {
        oldt = MaybeUninit::<termios>::uninit().assume_init();
        tcgetattr(STDIN_FILENO, &mut oldt);
        newt = oldt;
        newt.c_lflag &= !(ICANON | ECHO);
        tcsetattr(STDIN_FILENO, TCSANOW, &newt);
    }

    // Init game
    let mut game = Screen::new(10, 20);
    let mut current_shape = Shape::new(game.rows as i32);
    let mut next_shape = Shape::new(game.rows as i32);
    game.put(&current_shape);
    let (mut new_scores, mut new_lines) = (0, 0);
    let (mut scores, mut lines) = (0, 0usize);

    // Game loop
    let mut game_loop = true;
    while game_loop {
        // Print game
        game.print(scores, lines, &next_shape);

        // Input
        // 20 inputs per input time (input time in usleep)
        for _ in 0..20 {
            unsafe {
                usleep(((700_000 - (lines / 10) * 1000) / 60) as u32);

                // Magic
                let mut tv = timeval {
                    tv_sec: 0,
                    tv_usec: 0,
                };
                let mut fds = MaybeUninit::<fd_set>::uninit().assume_init();
                FD_ZERO(&mut fds);
                FD_SET(STDIN_FILENO, &mut fds);
                select(
                    STDIN_FILENO + 1,
                    &mut fds,
                    core::ptr::null_mut(),
                    core::ptr::null_mut(),
                    &mut tv,
                );
                if FD_ISSET(STDIN_FILENO, &fds as *const fd_set) {
                    // What key did hit
                    let mut key = getchar();
                    if key == 27 {
                        // ANSI Escape sequence "27 91 Key_code"
                        getchar(); // skip 91
                        key = getchar();
                    }
                    match key {
                        // Left array
                        68 => {
                            current_shape = game.move_side(&mut current_shape, -1);
                            game.print(scores, lines, &next_shape);
                        }
                        // Right array
                        67 => {
                            current_shape = game.move_side(&mut current_shape, 1);
                            game.print(scores, lines, &next_shape);
                        }
                        // Up array
                        65 => {
                            current_shape = game.rotate(&mut current_shape, 1);
                            game.print(scores, lines, &next_shape);
                        }
                        // Down array
                        66 => {
                            current_shape = game.rotate(&mut current_shape, -1);
                            game.print(scores, lines, &next_shape);
                        }
                        // Drop
                        32 => {
                            break;
                        }
                        _ => (),
                    }
                }
            }
        }

        // Update and print
        match game.move_down(&mut current_shape) {
            Err(()) => {
                game_over(scores, lines);
                game_loop = false;
            },
            Ok(result) => {
                let result_shape;
                let current_or_next;
                (result_shape, current_or_next, new_scores, new_lines) = result;

                // current shape does not change to new shape
                if !current_or_next {
                    current_shape = result_shape;
                } else {
                    current_shape = next_shape;
                    next_shape = result_shape;
                }
            }
        }
        scores += new_scores;
        lines += new_lines;
    }

    unsafe {
        tcsetattr(STDIN_FILENO, TCSANOW, &oldt);
    }
    0
}

fn game_over(scores: usize, lines: usize) {
    unsafe {
        system("clear\0".as_ptr() as Cstr);
        printf("\n _____   ___  ___  ___ _____   _____  _   _ ___________\n\0".as_ptr() as Cstr);
        printf("|  __ \\ / _ \\ |  \\/  ||  ___| |  _  || | | |  ___| ___ \\ \n\0".as_ptr() as Cstr);
        printf("| |  \\// /_\\ \\| .  . || |__   | | | || | | | |__ | |_/ /\n\0".as_ptr() as Cstr);
        printf("| | __ |  _  || |\\/| ||  __|  | | | || | | |  __||    / \n\0".as_ptr() as Cstr);
        printf("| |_\\ \\| | | || |  | || |___  \\ \\_/ /\\ \\_/ / |___| |\\ \\ \n\0".as_ptr() as Cstr);
        printf("\\____/\\_| |_/\\_|  |_/\\____/   \\___/  \\___/\\____/\\_| \\_|\n\0".as_ptr() as Cstr);
        printf("\n\nYOUR SCORES: %d; YOUR LINES: %d;\n\0".as_ptr() as Cstr, scores, lines);
    }
}

// For rust compiler
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
