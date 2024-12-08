use sdl2::{self, rect::Rect, render::WindowCanvas};

pub struct queen_struct {
    pub rank: i32,
    pub file: i32,
    pub dead: bool,
}

pub fn load() -> Vec<queen_struct> {
    let wq = queen_struct {
        rank: 1,
        file: 4,
        dead: false,
    };
    let bq = queen_struct {
        rank: 8,
        file: 4,
        dead: false,
    };
    vec![wq, bq]
}

pub fn render(
    canvas: &mut WindowCanvas,
    texture: &sdl2::render::Texture,
    _struct: &Vec<queen_struct>,
) {
    let rank_to_y = |rank: i32| -> i32 { 64 * (8 - rank) };

    //White queen
    let target = Rect::new(
        64 * _struct[0].file + 77,
        rank_to_y(_struct[0].rank) + 6,
        75,
        75,
    );
    let src = Rect::new(104, 0, 120, 120);
    canvas.copy(&texture, src, target).unwrap();

    //Black queen
    let target = Rect::new(
        64 * _struct[1].file + 77,
        rank_to_y(_struct[1].rank),
        75,
        75,
    );
    let src = Rect::new(104, 105, 120, 120);
    canvas.copy(&texture, src, target).unwrap();
}
