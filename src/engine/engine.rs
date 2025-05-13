extern crate sdl2;

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
    cells: Vec<Cell>,
    _food: Vec<Food>,
    rng: ThreadRng
}

impl<'e> Engine<'e> {
    pub fn new(canvas: &'e mut Canvas<Window>) -> Self {
        Self { 
            canvas, 
            cells: Vec::new(),
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
        self.cells.push(cell);
    }

    pub fn update_cells(&mut self) {
        for cell in self.cells.as_mut_slice() {
            let x = self.rng.random_range(-1..=1);
            let y = self.rng.random_range(-1..=1);
            cell.move_by(
                Vec2::new(x, y),
                self.canvas.window().size()
            );
        }
    }

    pub fn draw_cells(&mut self) {
        self.cells.iter().for_each(|cell | {
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
