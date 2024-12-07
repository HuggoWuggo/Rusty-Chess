use sdl2::{self, rect::Rect, render::WindowCanvas};

pub struct knight_struct {
    rank: i32,
    file: i32,
    dead: bool,
}

pub fn load() -> Vec<knight_struct> {
    let wlk = knight_struct {
        rank: 1,
        file: 2,
        dead: false,
    };
    let blk = knight_struct {
        rank: 8,
        file: 2,
        dead: false,
    };
    let wrk = knight_struct {
        rank: 1,
        file: 7,
        dead: false,
    };
    let brk = knight_struct {
        rank: 8,
        file: 7,
        dead: false,
    };
    vec![wlk, wrk, blk, brk]
}

pub fn render(
    canvas: &mut WindowCanvas,
    texture: &sdl2::render::Texture,
    _struct: &Vec<knight_struct>,
) {
    //White left knight
    let target = Rect::new(205, 455, 75, 75);
    let src = Rect::new(318, 0, 120, 120);
    canvas.copy(&texture, src, target).unwrap();

    //White right knight
    let target = Rect::new(525, 455, 75, 75);
    canvas.copy(&texture, src, target).unwrap();

    //Black left knight
    let target = Rect::new(205, 2, 75, 75);
    let src = Rect::new(318, 105, 120, 120);
    canvas.copy(&texture, src, target).unwrap();

    //Black right knight
    let target = Rect::new(525, 2, 75, 75);
    canvas.copy(&texture, src, target).unwrap();
}
