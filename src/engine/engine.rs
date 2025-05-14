extern crate sdl2;

use::std::cell::RefCell;

use rand::rngs::ThreadRng;
use rand::{Rng};
use sdl2::pixels::Color;
use sdl2::{render::Canvas, video::Window};
use sdl2::gfx::primitives;

use super::cell::Cell;
use super::food::Food;
use crate::physics::object::Object;
use crate::physics::vec2::Vec2;

pub struct Engine<'e> {
    pub canvas: &'e mut Canvas<Window>,
    cells: RefCell<Vec<Cell>>,
    _food: Vec<Food>,
    rng: ThreadRng
}

impl<'e> Engine<'e> {
    pub fn new(canvas: &'e mut Canvas<Window>) -> Self {
        Self { 
            canvas, 
            cells: RefCell::new(Vec::new()),
            _food: Vec::new(),
            rng: rand::rng()
        }
    }

    pub fn rand_spawn_init(&mut self, cell_amount: u8) {
        let (x, y) = self.canvas.window().drawable_size();
        for _ in 0..cell_amount {
            let a = self.rng.random_range(10..=20);
            let b = self.rng.random_range(0..x as i16);
            let c = self.rng.random_range(0..y as i16);
            self.add_cell(Cell::new(
                100, 
                100, 
                Object::new(
                    a, 
                    Vec2::new(b, c)
                )
            ));
        }
    } 

    pub fn add_cell(&mut self, cell: Cell) {
        self.cells.borrow_mut().push(cell);
    }

    fn update_cell(&mut self, idx: usize) {
        let x = self.rng.random_range(-1..=1);
        let y = self.rng.random_range(-1..=1);

        let mut cells = self.cells.borrow_mut();
        let tmp = Object::new(cells[idx].object.size, cells[idx].object.location + Vec2::new(x, y));
        
        // Does not update cell value at finding itself or colliding with a cell.
        let mut i = 0;
        while i < cells.len() {
            match tmp.collision(&cells[i].object, 1.25) {
                Ok(b) => if !b {
                    return;
                }
                Err(()) => ()
            }
            i += 1;
        }

        cells[idx].move_by(
            Vec2::new(x, y), 
            self.canvas.window().drawable_size()
        );
    }

    pub fn update_cells(&mut self) {
        let mut idx = 0;
        while idx < self.cells.borrow().len() {
            self.update_cell(idx);
            idx += 1
        }
    }

    pub fn draw_cells(&mut self) {
        self.cells.borrow_mut().iter().for_each(|cell | {
            primitives::DrawRenderer::filled_circle(
                self.canvas, 
                cell.object.location.x as i16, 
                cell.object.location.y as i16, 
                cell.object.size, 
                Color::BLACK
            ).unwrap();
        });
        
    }
}
