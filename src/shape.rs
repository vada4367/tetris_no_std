// Import Box<T> and Vec<T>
use alloc::{boxed::Box, vec, vec::Vec};

// Import libc types
type Cstr = *const i8;

extern crate libc;
use libc::{c_char, c_int};

#[cfg(target_os = "linux")]
#[link(name = "c")]

// Link libc funcs
extern "C" {
    fn rand() -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
}

#[derive(Clone)]
pub struct Shape {
    shape: Shapes,
    pub x: i32,
    pub y: i32,
    pub dx: usize,
    pub dy: usize,
    pub rotate: i32,
    pub canvas: Box<[bool]>,
}

impl Shape {
    pub fn new(sc_dx: i32) -> Self {
        let rand_num = unsafe { rand() } as usize;
        match _SHAPES[rand_num % _SHAPES.len()] {
            Shapes::Tshape => {
                return Self {
                    shape: Shapes::Tshape,
                    x: sc_dx / 2 - 2,
                    y: 0,
                    dx: 3,
                    dy: 3,
                    rotate: 0,
                    canvas: vec![false, false, false, true, true, true, false, true, false]
                        .into_boxed_slice(),
                }
            }
            Shapes::Ishape => {
                return Self {
                    shape: Shapes::Ishape,
                    x: sc_dx / 2 - 2,
                    y: 0,
                    dx: 4,
                    dy: 4,
                    rotate: 0,
                    canvas: vec![
                        false, false, false, false, true, true, true, true, false, false, false,
                        false, false, false, false, false,
                    ]
                    .into_boxed_slice(),
                }
            }
            Shapes::Oshape => {
                return Self {
                    shape: Shapes::Oshape,
                    x: sc_dx / 2 - 2,
                    y: 0,
                    dx: 2,
                    dy: 2,
                    rotate: 0,
                    canvas: vec![true, true, true, true].into_boxed_slice(),
                }
            }
            Shapes::Sshape => {
                return Self {
                    shape: Shapes::Sshape,
                    x: sc_dx / 2 - 2,
                    y: 0,
                    dx: 3,
                    dy: 3,
                    rotate: 0,
                    canvas: vec![false, true, true, true, true, false, false, false, false]
                        .into_boxed_slice(),
                }
            }
            Shapes::Zshape => {
                return Self {
                    shape: Shapes::Zshape,
                    x: sc_dx / 2 - 2,
                    y: 0,
                    dx: 3,
                    dy: 3,
                    rotate: 0,
                    canvas: vec![true, true, false, false, true, true, false, false, false]
                        .into_boxed_slice(),
                }
            }
            Shapes::Jshape => {
                return Self {
                    shape: Shapes::Jshape,
                    x: sc_dx / 2 - 2,
                    y: 0,
                    dx: 3,
                    dy: 3,
                    rotate: 0,
                    canvas: vec![true, false, false, true, true, true, false, false, false]
                        .into_boxed_slice(),
                }
            }
            Shapes::Lshape => {
                return Self {
                    shape: Shapes::Lshape,
                    x: sc_dx / 2 - 2,
                    y: 0,
                    dx: 3,
                    dy: 3,
                    rotate: 0,
                    canvas: vec![false, false, true, true, true, true, false, false, false]
                        .into_boxed_slice(),
                }
            }
        }
    }

    pub fn print(&self) {
        for i in 0..(self.dx * self.dy) as usize {
            unsafe {
                if i % self.dx == 0 {
                    printf("\n\0".as_ptr() as Cstr);
                }
                if self.canvas[i] {
                    printf("## \0".as_ptr() as Cstr);
                } else {
                    printf(".. \0".as_ptr() as Cstr);
                }
            }
        }
        unsafe {
            printf("\n\0".as_ptr() as Cstr);
        }
    }

    // Just examples:
    //
    //                 .. ## ## <- this is max x
    // this is min x-> ## ## ..
    //                 .. .. ..
    //
    //   .. .. ## ..
    //   .. .. ## ..
    //   .. .. ## ..
    //   .. .. ## ..
    //          ^
    //  this is | max and min x
    pub fn minmax(&self) -> (i32, i32) {
        let mut minx = self.dx as i32;
        let mut maxx = 0i32;
        for i in 0..self.dx {
            for j in 0..self.dy {
                if self.canvas[i + j * self.dx] && (i as i32) < minx {
                    minx = i as i32;
                }
                if self.canvas[i + j * self.dx] && (i as i32) > maxx {
                    maxx = i as i32;
                }
            }
        }
        (minx, maxx)
    }

    // Rotate shapes
    pub fn rotate(&mut self, rotate: i32) -> Self {
        self.rotate += rotate;
        if self.rotate < 0 {
            self.rotate += 4;
        }
        if self.rotate > 3 {
            self.rotate -= 4;
        }

        let allrotates: Vec<Vec<bool>>;
        match self.shape {
            Shapes::Tshape => {
                allrotates = vec![
                    vec![false, false, false, true, true, true, false, true, false],
                    vec![false, true, false, true, true, false, false, true, false],
                    vec![false, true, false, true, true, true, false, false, false],
                    vec![false, true, false, false, true, true, false, true, false],
                ];
            }
            Shapes::Ishape => {
                allrotates = vec![
                    vec![
                        false, false, false, false, true, true, true, true, false, false, false,
                        false, false, false, false, false,
                    ],
                    vec![
                        false, false, true, false, false, false, true, false, false, false, true,
                        false, false, false, true, false,
                    ],
                    vec![
                        false, false, false, false, false, false, false, false, true, true, true,
                        true, false, false, false, false,
                    ],
                    vec![
                        false, true, false, false, false, true, false, false, false, true, false,
                        false, false, true, false, false,
                    ],
                ];
            }
            Shapes::Oshape => {
                allrotates = vec![
                    vec![true, true, true, true],
                    vec![true, true, true, true],
                    vec![true, true, true, true],
                    vec![true, true, true, true],
                ];
            }
            Shapes::Sshape => {
                allrotates = vec![
                    vec![false, true, true, true, true, false, false, false, false],
                    vec![false, true, false, false, true, true, false, false, true],
                    vec![false, false, false, false, true, true, true, true, false],
                    vec![true, false, false, true, true, false, false, true, false],
                ];
            }
            Shapes::Zshape => {
                allrotates = vec![
                    vec![true, true, false, false, true, true, false, false, false],
                    vec![false, false, true, false, true, true, false, true, false],
                    vec![false, false, false, true, true, false, false, true, true],
                    vec![false, true, false, true, true, false, true, false, false],
                ];
            }
            Shapes::Jshape => {
                allrotates = vec![
                    vec![true, false, false, true, true, true, false, false, false],
                    vec![false, true, true, false, true, false, false, true, false],
                    vec![false, false, false, true, true, true, false, false, true],
                    vec![false, true, false, false, true, false, true, true, false],
                ];
            }
            Shapes::Lshape => {
                allrotates = vec![
                    vec![false, false, true, true, true, true, false, false, false],
                    vec![false, true, false, false, true, false, false, true, true],
                    vec![false, false, false, true, true, true, true, false, false],
                    vec![true, true, false, false, true, false, false, true, false],
                ];
            }
        }

        self.canvas = allrotates[self.rotate as usize].clone().into_boxed_slice();
        self.clone()
    }
}

// All shapes
#[derive(Clone, Copy)]
enum Shapes {
    Tshape,
    Ishape,
    Oshape,
    Sshape,
    Zshape,
    Jshape,
    Lshape,
}

const _SHAPES: [Shapes; 7] = [
    Shapes::Tshape,
    Shapes::Ishape,
    Shapes::Oshape,
    Shapes::Sshape,
    Shapes::Zshape,
    Shapes::Jshape,
    Shapes::Lshape,
];
