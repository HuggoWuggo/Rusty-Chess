use sdl2::{self, rect::Rect, render::WindowCanvas};

pub struct knight_struct {
    pub rank: i32,
    pub file: i32,
    pub dead: bool,
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
    vec![wlk, blk, wrk, brk]
}

pub fn render(
    canvas: &mut WindowCanvas,
    texture: &sdl2::render::Texture,
    _struct: &Vec<knight_struct>,
) {
    let rank_to_y = |rank: i32| -> i32 { 64 * (8 - rank) };

    //White left knight
    let target = Rect::new(
        64 * _struct[0].file + 77,
        rank_to_y(_struct[0].rank) + 6,
        75,
        75,
    );
    let src = Rect::new(318, 0, 120, 120);
    canvas.copy(&texture, src, target).unwrap();

    //White right knight
    let target = Rect::new(
        64 * _struct[2].file + 77,
        rank_to_y(_struct[2].rank) + 6,
        75,
        75,
    );
    canvas.copy(&texture, src, target).unwrap();

    //Black left knight
    let target = Rect::new(
        64 * _struct[1].file + 77,
        rank_to_y(_struct[1].rank),
        75,
        75,
    );
    let src = Rect::new(318, 105, 120, 120);
    canvas.copy(&texture, src, target).unwrap();

    //Black right knight
    let target = Rect::new(
        64 * _struct[3].file + 77,
        rank_to_y(_struct[3].rank),
        75,
        75,
    );
    canvas.copy(&texture, src, target).unwrap();
}
