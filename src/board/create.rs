use super::grid::*;
use crate::pieces::*;

use sdl2::{
    self,
    event::Event,
    image::{InitFlag, LoadTexture},
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture},
    video::Window,
    Sdl,
};

pub const SQUARE_SIZE: i32 = 64;

pub fn new() -> Result<(), String> {
    let g = Grid::new();
    let sdl2_context = sdl2::init()?;
    let video_subsystem = sdl2_context.video()?;
    let window = video_subsystem
        .window("Rusty-Chess", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| format!("{e}"))?;
    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .load_texture("src/pieces/sprites/pieces.png")
        .expect("Failed to load tetxure path");

    let _u = update(sdl2_context, canvas, g, texture);
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    Ok(())
}

pub fn update(
    sdl2_context: Sdl,
    mut canvas: Canvas<Window>,
    mut g: Grid,
    _texture: Texture<'_>,
) -> Result<(), String> {
    let _rooks = rook::load();
    let _knights = knight::load();

    'app: loop {
        for event in sdl2_context.event_pump()?.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'app;
            }
        }

        //Update

        //Render
        //let mut grid: Vec<Rect> = Vec::new();
        if g.squares.is_empty() {
            for row in 0..8 {
                for col in 0..8 {
                    let x = col * SQUARE_SIZE + 144;
                    let y = row * SQUARE_SIZE + 10;
                    g.squares
                        .push(Rect::new(x, y, SQUARE_SIZE as u32, SQUARE_SIZE as u32));
                }
            }
        }

        // Set the background color
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // Draw the grid
        for (i, square) in g.squares.iter().enumerate() {
            // Alternate colors for a checkerboard pattern
            if (i / 8 + i % 8) % 2 == 0 {
                canvas.set_draw_color(Color::WHITE);
            } else {
                canvas.set_draw_color(Color::GRAY);
            }
            canvas.fill_rect(*square)?;
        }

        rook::render(&mut canvas, &_texture, &_rooks);
        knight::render(&mut canvas, &_texture, &_knights);

        // Present the canvas to the screen
        canvas.present();
    }
    Ok(())
}
