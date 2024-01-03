// Import Shapes structs
use crate::Shape;

// For all system (for android *const u8)
type Cstr = *const i8;

// Import libc types
extern crate libc;
use libc::{c_char, c_int};

#[cfg(target_os = "linux")]
#[link(name = "c")]

// Link libc funcs
extern "C" {
    fn system(s: *const c_char) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
}

// Import Box<T> and Vec<T>
extern crate alloc;
use alloc::{boxed::Box, vec, vec::Vec};

pub struct Screen {
    pub rows: usize,
    pub cols: usize,
    pub sc: Box<[bool]>,
}

impl Screen {
    pub fn new(r: usize, c: usize) -> Self {
        Self {
            rows: r,
            cols: c,
            sc: vec![false; r * c].into_boxed_slice(),
        }
    }

    pub fn print(&self, scores: usize, lines: usize, shape: &Shape) {
        unsafe {
            // Clear console
            system("clear\0".as_ptr() as Cstr);
            // Move cursor to 0; 0
            printf("\x1b[H\0".as_ptr() as Cstr);
        }

        for i in 0..self.rows * self.cols {
            unsafe {
                if i % self.rows == 0 {
                    printf("\n\0".as_ptr() as Cstr);
                }

                if self.sc[i] {
                    printf("[] \0".as_ptr() as Cstr);
                } else {
                    printf(".. \0".as_ptr() as Cstr);
                }
            }
        }

        unsafe {
            printf(
                "\nSCORES: %d; LINES: %d\n\nNEXT SHAPE:\0".as_ptr() as Cstr,
                scores,
                lines,
            );
            shape.print();
        }
    }

    // Put shape to screen
    // (unsafe, u need to use updatable for safe use put)
    pub fn put(&mut self, shape: &Shape) {
        for i in 0..shape.dx {
            for j in 0..shape.dy {
                if shape.canvas[i + j * shape.dx]
                    && !self.sc[(shape.x as usize + i) + (shape.y as usize + j) * self.rows]
                {
                    self.sc[(shape.x as usize + i) + (shape.y as usize + j) * self.rows] = true;
                }
            }
        }
    }

    // Can shape be putted to screen?
    fn updatable(&self, shape: &Shape) -> Result<(), ()> {
        for i in 0..shape.dx {
            for j in 0..shape.dy {
                if shape.canvas[i + j * shape.dx] {
                    // End of the screen
                    if shape.x as usize + i + (shape.y as usize + j) * self.rows
                        >= self.rows * self.cols
                    {
                        return Err(());
                    }

                    // Collision with another figure
                    if self.sc[(shape.x as usize + i) + (shape.y as usize + j) * self.rows] {
                        return Err(());
                    }
                }
            }
        }

        Ok(())
    }

    // Delete shape from screen
    fn delete(&mut self, shape: &Shape) {
        for i in 0..shape.dx {
            for j in 0..shape.dy {
                if shape.canvas[i + j * shape.dx] {
                    self.sc[(shape.x as usize + i) + (shape.y as usize + j) * self.rows] = false;
                }
            }
        }
    }

    // Return lines which need to delete
    fn full_lines(&self) -> Vec<usize> {
        let mut result = vec![];
        let mut full;
        for i in 0..self.cols {
            full = true;
            for j in 0..self.rows {
                if !self.sc[j + i * self.rows] {
                    full = false;
                    break;
                }
            }
            if full {
                result.push(i);
            }
        }
        result
    }

    // Delete line
    fn clear_line(&mut self, line: usize) {
        for i in 0..self.rows {
            self.sc[i] = false;
        }
        for i in 1..line {
            for j in 0..self.rows {
                self.sc[j + (line - i + 1) * self.rows] = self.sc[j + (line - i) * self.rows];
            }
        }
    }

    // Delete all lines which need to delete
    fn clear_lines(&mut self) -> (usize, usize) {
        let lines = self.full_lines();
        for i in 0..lines.len() {
            self.clear_line(lines[i]);
        }
        return (lines.len() * lines.len() * 10, lines.len());
    }

    // Update game loop
    // (move shape down, can return game over, can make new shape)
    pub fn move_down(&mut self, shape: &mut Shape) -> Result<(Shape, bool, usize, usize), ()> {
        self.delete(shape);
        shape.y += 1;
        let (mut lines, mut scores) = (0, 0);

        match self.updatable(&shape) {
            Ok(()) => {
                self.put(&shape);
            }
            Err(()) => {
                shape.y -= 1;
                self.put(shape);
                (scores, lines) = self.clear_lines();
                let new_shape = Shape::new(self.rows as i32);
                match self.updatable(&new_shape) {
                    Err(()) => return Err(()),
                    Ok(()) => return Ok((new_shape, true, scores, lines)),
                }
            }
        }

        Ok((shape.clone(), false, scores, lines))
    }

    // Rotate figure
    pub fn rotate(&mut self, shape: &mut Shape, rotate: i32) -> Shape {
        self.delete(shape);

        let (minx, maxx) = shape.clone().rotate(rotate).minmax();
        if shape.x + minx < 0 || shape.x + maxx > self.rows as i32 - 1 {
            self.put(shape);
            return shape.clone();
        }

        match self.updatable(&(&mut shape.clone()).rotate(rotate)) {
            Ok(()) => {
                self.put(&shape.rotate(rotate));
            }
            Err(()) => {
                self.put(shape);
            }
        }

        shape.clone()
    }

    // Move figure left/right
    pub fn move_side(&mut self, shape: &mut Shape, side: i32) -> Shape {
        self.delete(shape);
        shape.x += side;

        let (minx, maxx) = shape.minmax();

        if shape.x + minx < 0 {
            shape.x = -minx;
        }
        if shape.x + maxx > self.rows as i32 - 1 {
            shape.x = self.rows as i32 - maxx - 1;
        }

        match self.updatable(&shape) {
            Ok(()) => {
                self.put(&shape);
            }
            Err(()) => {
                shape.x -= side;
                self.put(shape);
            }
        }

        shape.clone()
    }
}
