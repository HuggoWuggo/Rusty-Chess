use sdl2::rect;
pub struct Grid {
    pub squares: Vec<rect::Rect>,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            squares: Vec::new(),
        }
    }
}
