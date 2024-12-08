use sdl2::{self, rect::Rect, render::WindowCanvas};

pub struct rook_struct {
    pub rank: i32,
    pub file: i32,
    pub dead: bool,
}

pub fn load() -> Vec<rook_struct> {
    let mut wlr = rook_struct {
        rank: 1,
        file: 1,
        dead: false,
    };
    let mut blr = rook_struct {
        rank: 8,
        file: 1,
        dead: false,
    };
    let mut wrr = rook_struct {
        rank: 1,
        file: 8,
        dead: false,
    };
    let mut brr = rook_struct {
        rank: 8,
        file: 8,
        dead: false,
    };
    let mut v = vec![wlr, blr, wrr, brr];
    v
}

pub fn render(
    canvas: &mut WindowCanvas,
    texture: &sdl2::render::Texture,
    _struct: &Vec<rook_struct>,
) {
    let rank_to_y = |rank: i32| -> i32 { 64 * (8 - rank) };

    // White left rook
    let target = Rect::new(
        64 * (_struct[0].file) + 74,
        rank_to_y(_struct[0].rank) + 6,
        75,
        75,
    );
    let src = Rect::new(420, 0, 120, 120);
    canvas.copy(&texture, src, target).unwrap();

    // White right rook
    let target = Rect::new(
        64 * (_struct[2].file) + 74,
        rank_to_y(_struct[2].rank) + 6,
        75,
        75,
    );
    canvas.copy(&texture, src, target).unwrap();

    // Black left rook
    let target = Rect::new(
        64 * (_struct[1].file) + 74,
        rank_to_y(_struct[1].rank),
        75,
        75,
    );
    let src = Rect::new(420, 105, 120, 120);
    canvas.copy(&texture, src, target).unwrap();

    // Black right rook
    let target = Rect::new(
        64 * (_struct[3].file) + 74,
        rank_to_y(_struct[3].rank),
        75,
        75,
    );
    canvas.copy(&texture, src, target).unwrap();
}
