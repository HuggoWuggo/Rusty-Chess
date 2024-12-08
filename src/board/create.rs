use std::time::Duration;

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
    let _bishops = bishop::load();
    let _pawns = pawn::load();
    let _queens = queen::load();
    let _kings = king::load();

    let grid_offset_x = 144; // Offset of the grid from the window's left
    let grid_offset_y = 10; // Offset of the grid from the window's top

    'app: loop {
        for event in sdl2_context.event_pump()?.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'app;
            }
            if let Event::MouseButtonDown { x, y, .. } = event {
                if x >= grid_offset_x && y >= grid_offset_y {
                    let grid_x = x - grid_offset_x;
                    let grid_y = y - grid_offset_y;

                    let file = grid_x / SQUARE_SIZE + 1;
                    let rank = 7 - (grid_y / SQUARE_SIZE) + 1;

                    if file >= 0 && file <= 8 && rank >= 0 && rank <= 8 {
                        //println!("Clicked on rank: {}, file: {}", rank, file);
                    } else {
                        println!("Click outside the grid");
                    }
                    for r in &_rooks {
                        if r.rank == rank && r.file == file && r.dead == false {
                            println!("CLICKED ON A ROOK!");
                        }
                    }
                    for k in &_knights {
                        if k.rank == rank && k.file == file && k.dead == false {
                            println!("CLICKED ON A KNIGHT!");
                        }
                    }
                    for b in &_bishops {
                        if b.rank == rank && b.file == file && b.dead == false {
                            println!("CLICKED ON A BISHOP!");
                        }
                    }
                    for p in &_pawns {
                        if p.rank == rank && p.file == file && p.dead == false {
                            println!("CLICKED ON A PAWN!");
                        }
                    }
                    for q in &_queens {
                        if q.rank == rank && q.file == file && q.dead == false {
                            println!("CLICKED ON A QUEEN!");
                        }
                    }
                    for kn in &_kings {
                        if kn.rank == rank && kn.file == file && kn.dead == false {
                            println!("CLICKED ON A KING");
                        }
                    }
                }
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
        bishop::render(&mut canvas, &_texture, &_bishops);
        pawn::render(&mut canvas, &_texture, &_pawns);
        queen::render(&mut canvas, &_texture, &_queens);
        king::render(&mut canvas, &_texture, &_kings);

        // Present the canvas to the screen
        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }
    Ok(())
}
