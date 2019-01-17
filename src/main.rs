extern crate piston_window;
extern crate graphics;

use core::cell::RefCell;
use piston_window::*;
use graphics::Graphics;

//---------------------------------------
// CONSTANTS
//---------------------------------------
/* -- COLORS -- */
const BACKGROUND_C: [f32; 4] = [0.5, 0.5, 0.5, 1.0];

const WINDOW_WIDTH: u32 = 400;
const WINDOW_HEIGHT: u32 = 400;
const FIELD_WIDTH: u32 = 200;
const FIELD_HEIGHT: u32 = 400;
const GRID_WIDTH: u32 = 10;
const GRID_HEIGHT: u32 = 20;

//---------------------------------------
// WINDOW
//---------------------------------------
struct Window {
    width: u32,
    height: u32,
    pwindow: PistonWindow,
}

impl Window {
    fn new(_width: u32, _height: u32) -> Window {
        let opengl = OpenGL::V3_2;
        Window {
            width: _width,
            height: _height,
            pwindow: WindowSettings::new("Tetris", (_width, _height))
                .exit_on_esc(true)
                .opengl(opengl)
                .build()
                .unwrap(),
        }
    }

    fn update(&mut self, ecs: &ECS) {
        if let Some(e) = self.pwindow.next() {
            if let Some(r_a) = e.render_args() {
                self.pwindow.draw_2d(&e, |c, g| {
                    clear(BACKGROUND_C, g);

                    ecs.draw(&c, &g);
                });
            }
            if let Some(u_a) = e.update_args() {
                ecs.update(u_a.dt);
            }
        }
    }

}

//---------------------------------------
// ENTITY_COMPONENT_SYSTEM
//---------------------------------------

struct ECS {
}

impl ECS {

    fn new() -> ECS {
        ECS {
            
        }
    }

    fn draw(&self, c: &impl Transformed, g: &impl Graphics) {
    }

    fn update(&self, dt: f64) {
        println!("updating ecs: {}", dt);
    }
}

//---------------------------------------
// ENTITIES
//---------------------------------------
struct Field {
    x: u32,
    y: u32,

    width: u32,
    height: u32,
    grid: [u8; (GRID_WIDTH * GRID_HEIGHT) as usize],
}

impl Field {
    fn new() -> Field {
        Field {
            x: (WINDOW_WIDTH - FIELD_WIDTH)/2,
            y: (WINDOW_HEIGHT- FIELD_HEIGHT)/2,
            width: FIELD_WIDTH,
            height: FIELD_HEIGHT,
            grid: [0; (GRID_WIDTH * GRID_HEIGHT) as usize],
        }
    }
    
    fn draw(&self, t: [[f64; 3]; 2], g: &mut impl Graphics) {
        let size : u32 = FIELD_WIDTH/GRID_WIDTH;
        for x in 0..(GRID_WIDTH - 1) {
            for y in 0..(GRID_HEIGHT - 1) {
                let pos = (x + y * GRID_WIDTH) as usize;
                if self.grid[pos] == 1 {
                    rectangle([1.0, 0.0, 0.0, 1.0],
                              [(self.x + x * GRID_WIDTH) as f64,
                               (self.y + y * GRID_WIDTH) as f64,
                               GRID_WIDTH as f64,
                               GRID_WIDTH as f64],
                              t, g);
                } else if self.grid[pos] == 0 {
                    rectangle([1.0, 0.0, 0.0, 1.0],
                              [(self.x + x * GRID_WIDTH) as f64,
                               (self.y + y * GRID_WIDTH) as f64,
                               GRID_WIDTH as f64,
                               GRID_WIDTH as f64],
                              t, g);
                }
            }
        }
    }
}

fn main() {
    println!("starting tetris");

    let mut window = Window::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    let ecs = ECS::new();

    loop {
        window.update(&ecs);
    }
}
