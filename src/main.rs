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
// UTIL
//---------------------------------------

struct Vec2 {
    x: u32,
    y: u32
}

impl Vec2 {
    fn new(x: u32, y: u32) -> Vec2 {
        Vec2 {
            x: x,
            y: y,
        }
    }
}

struct Rec {
    pos: Vec2,
    dim: Vec2,
}

impl Rec {
    fn new(pos: Vec2, dim: Vec2) -> Rec {
        Rec {
            pos: pos,
            dim: dim,
        }
    }
}

//---------------------------------------
// WINDOW
//---------------------------------------
struct Window {
    width: u32,
    height: u32,
    pwindow: PistonWindow,
    glyphs: Glyphs,
}

impl Window {
    fn new(_width: u32, _height: u32) -> Window {
        let opengl = OpenGL::V3_2;

        let window: PistonWindow = WindowSettings::new("Tetris", (_width, _height))
            .exit_on_esc(true)
            .opengl(opengl)
            .build()
            .unwrap();

        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap();
        println!("{:?}", assets);
        let ref font = assets.join("BitFont.ttf");
        let factory = window.factory.clone();
        let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

        Window {
            width: _width,
            height: _height,
            pwindow: window,
            glyphs: glyphs,
        }
    }

    fn start(&mut self, em: &mut Entity_Manager) {
        while let Some(e) = self.pwindow.next() {
            let glyphs: &mut Glyphs = &mut self.glyphs;
            if let Some(r_a) = e.render_args() {
                self.pwindow.draw_2d(&e, |c, g| {
                    clear(BACKGROUND_C, g);

                    em.draw(&c, g, glyphs);
                });
            }
            if let Some(u_a) = e.update_args() {
                em.update(u_a.dt, );
            }
        }
    }

}

//---------------------------------------
// ENTITY_COMPONENT_SYSTEM
//---------------------------------------

struct Entity_Manager {
    fps_view: FPS_View,
    field: Field,
}

impl Entity_Manager {

    fn new() -> Entity_Manager {
        Entity_Manager {
            fps_view: FPS_View::new(),
            field: Field::new(),
        }
    }

    fn draw(&self, c: &Context, g: &mut G2d, glyphs: &mut Glyphs) {
        let bound = Rec::new(Vec2::new(0,0), Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT));

        self.field.draw(c, g, glyphs, &bound);
        self.fps_view.draw(c, g, glyphs, &bound);
    }

    fn update(&mut self, dt: f64) {
        self.field.update(dt);

        self.fps_view.update(dt);
    }
}

trait Entity {
    fn draw(&self, c: &Context, g:&mut G2d, glyphs: &mut Glyphs, bound: &Rec);
    fn update(&mut self, dt: f64);
}

//---------------------------------------
// ENTITIES
//---------------------------------------

/* -- FPS_VIEW -- */

struct FPS_View {
    fps: u32,
    color: [f32; 4],
}

impl FPS_View {
    fn new() -> FPS_View {
        FPS_View {
            fps: 0,
            color: [1.0, 0.0, 0.0, 1.0],
        }
    }
}

impl Entity for FPS_View {

    fn update(&mut self, dt: f64) {
        self.fps = (1.0 / dt) as u32;
    }

    fn draw(&self, c: &Context, g: &mut G2d, glyphs: &mut Glyphs, bound: &Rec){
        let mut fps_str: String = "FPS: ".to_owned();
        fps_str.push_str(&self.fps.to_string());
        text::Text::new_color(self.color, 10).draw(
            &fps_str,
            glyphs,
            &c.draw_state,
            c.transform.trans(10.0,20.0), g
        );
    }

}

/* -- FIELD -- */

struct Field {
    x: u32,
    y: u32,

    width: u32,
    height: u32,
    grid: [u8; (GRID_WIDTH * GRID_HEIGHT) as usize],

    move_time: f64,
    delta_move_time: f64,
    moves: u32,
}

impl Field {
    fn new() -> Field {
        Field {
            x: (WINDOW_WIDTH - FIELD_WIDTH)/2,
            y: (WINDOW_HEIGHT- FIELD_HEIGHT)/2,
            width: FIELD_WIDTH,
            height: FIELD_HEIGHT,
            grid: [0; (GRID_WIDTH * GRID_HEIGHT) as usize],

            move_time: 1.0,
            delta_move_time: 0.0,
            
            moves: 0,
        }
    }
}

impl Entity for Field {

    fn draw(&self, c: &Context, g: &mut G2d, glyphs: &mut Glyphs, bound: &Rec){
        let size : u32 = FIELD_WIDTH/GRID_WIDTH;
        for x in 0..(GRID_WIDTH - 1) {
            for y in 0..(GRID_HEIGHT - 1) {
                let pos = (x + y * GRID_WIDTH) as usize;
                if self.grid[pos] == 1 {
                    rectangle([0.0, 1.0, 0.0, 1.0],
                              [(self.x + x * GRID_WIDTH) as f64,
                               (self.y + y * GRID_WIDTH) as f64,
                               GRID_WIDTH as f64,
                               GRID_WIDTH as f64],
                              c.transform, g);
                } else if self.grid[pos] == 0 {
                    rectangle([1.0, 0.0, 0.0, 1.0],
                              [(self.x + x * GRID_WIDTH) as f64,
                               (self.y + y * GRID_WIDTH) as f64,
                               GRID_WIDTH as f64,
                               GRID_WIDTH as f64],
                              c.transform, g);
                }
            }
        }
    }

    fn update(&mut self, dt: f64) {
        self.delta_move_time += dt;
        if(self.delta_move_time >= self.move_time) {
            self.delta_move_time -= self.move_time;
            //do move
            println!("move");
            self.grid[(self.moves % (GRID_WIDTH * GRID_HEIGHT)) as usize] = 1;
                
            self.moves += 1;
        }
    }
}

fn main() {
    println!("starting tetris");

    let mut window = Window::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    let mut em = Entity_Manager::new();

    window.start(&mut em);
}
