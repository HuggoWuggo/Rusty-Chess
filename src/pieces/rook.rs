use sdl2::{self, rect::Rect, render::WindowCanvas};

pub struct rook_struct {
    rank: i32,
    file: i32,
    dead: bool,
}

pub fn load() -> Vec<rook_struct> {
    let wlr = rook_struct {
        rank: 1,
        file: 1,
        dead: false,
    };
    let blr = rook_struct {
        rank: 8,
        file: 1,
        dead: false,
    };
    let wrr = rook_struct {
        rank: 1,
        file: 8,
        dead: false,
    };
    let brr = rook_struct {
        rank: 8,
        file: 8,
        dead: false,
    };
    vec![wlr, wrr, blr, brr]
}

pub fn render(
    canvas: &mut WindowCanvas,
    texture: &sdl2::render::Texture,
    _struct: &Vec<rook_struct>,
) {
    //White left rook
    let target = Rect::new(64 * _struct[0].file + 74, 455, 75, 75);
    let src = Rect::new(420, 0, 120, 120);
    canvas.copy(&texture, src, target).unwrap();

    //White right rook
    let target = Rect::new(64 * _struct[1].file + 74, 455, 75, 75);
    canvas.copy(&texture, src, target).unwrap();

    //Black left rook
    let target = Rect::new(64 * _struct[2].file + 74, 2, 75, 75);
    let src = Rect::new(420, 105, 120, 120);
    canvas.copy(&texture, src, target).unwrap();

    //Black right rook
    let target = Rect::new(64 * _struct[3].file + 74, 2, 75, 75);
    canvas.copy(&texture, src, target).unwrap();
}
