use sdl2::{self, rect::Rect, render::WindowCanvas};

pub struct king_struct {
    pub rank: i32,
    pub file: i32,
    pub dead: bool,
}

pub fn load() -> Vec<king_struct> {
    let wk = king_struct {
        rank: 1,
        file: 5,
        dead: false,
    };
    let bk = king_struct {
        rank: 8,
        file: 5,
        dead: false,
    };
    vec![wk, bk]
}

pub fn render(
    canvas: &mut WindowCanvas,
    texture: &sdl2::render::Texture,
    _struct: &Vec<king_struct>,
) {
    let rank_to_y = |rank: i32| -> i32 { 64 * (8 - rank) };

    //White king
    let target = Rect::new(
        64 * _struct[0].file + 77,
        rank_to_y(_struct[0].rank) + 6,
        75,
        75,
    );
    let src = Rect::new(0, 0, 110, 110);
    canvas.copy(&texture, src, target).unwrap();

    //Black king
    let target = Rect::new(
        64 * _struct[1].file + 77,
        rank_to_y(_struct[1].rank),
        75,
        75,
    );
    let src = Rect::new(0, 105, 110, 120);
    canvas.copy(&texture, src, target).unwrap();
}
