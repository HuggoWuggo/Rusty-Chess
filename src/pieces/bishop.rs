use sdl2::{self, rect::Rect, render::WindowCanvas};

pub struct bishop_struct {
    pub rank: i32,
    pub file: i32,
    pub dead: bool,
}

pub fn load() -> Vec<bishop_struct> {
    let wlb = bishop_struct {
        rank: 1,
        file: 3,
        dead: false,
    };
    let blb = bishop_struct {
        rank: 8,
        file: 3,
        dead: false,
    };
    let wrb = bishop_struct {
        rank: 1,
        file: 6,
        dead: false,
    };
    let brb = bishop_struct {
        rank: 8,
        file: 6,
        dead: false,
    };
    vec![wlb, blb, wrb, brb]
}

pub fn render(
    canvas: &mut WindowCanvas,
    texture: &sdl2::render::Texture,
    _struct: &Vec<bishop_struct>,
) {
    let rank_to_y = |rank: i32| -> i32 { 64 * (8 - rank) };

    //White left bishop
    let target = Rect::new(
        64 * _struct[0].file + 77,
        rank_to_y(_struct[0].rank) + 6,
        75,
        75,
    );
    let src = Rect::new(209, 0, 120, 120);
    canvas.copy(&texture, src, target).unwrap();

    //White right bishop
    let target = Rect::new(
        64 * _struct[2].file + 77,
        rank_to_y(_struct[2].rank) + 6,
        75,
        75,
    );
    canvas.copy(&texture, src, target).unwrap();

    //Black left bishop
    let target = Rect::new(
        64 * _struct[1].file + 77,
        rank_to_y(_struct[1].rank),
        75,
        75,
    );
    let src = Rect::new(209, 105, 120, 120);
    canvas.copy(&texture, src, target).unwrap();

    //Black right bishop
    let target = Rect::new(
        64 * _struct[3].file + 77,
        rank_to_y(_struct[3].rank),
        75,
        75,
    );
    canvas.copy(&texture, src, target).unwrap();
}
