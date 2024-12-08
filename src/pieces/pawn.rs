use sdl2::{self, rect::Rect, render::WindowCanvas};

pub struct pawn_struct {
    pub rank: i32,
    pub file: i32,
    pub dead: bool,
}

pub fn load() -> Vec<pawn_struct> {
    let mut pawns: Vec<pawn_struct> = Vec::new();
    //White
    for i in 1..=8 {
        pawns.push(pawn_struct {
            rank: 2,
            file: i,
            dead: false,
        });
    }

    //Black
    for i in 1..=8 {
        pawns.push(pawn_struct {
            rank: 7,
            file: i,
            dead: false,
        });
    }
    pawns
}

pub fn render(
    canvas: &mut WindowCanvas,
    texture: &sdl2::render::Texture,
    _struct: &Vec<pawn_struct>,
) {
    let rank_to_y = |rank: i32| -> i32 { 64 * (8 - rank) };

    for i in 0..=7 {
        //White pawns
        let target = Rect::new(
            64 * _struct[i].file + 77,
            rank_to_y(_struct[i].rank) + 6,
            75,
            75,
        );
        let src = Rect::new(531, 0, 120, 120);
        canvas.copy(&texture, src, target).unwrap();
    }
    for i in 8..=15 {
        //Black pawns
        let target = Rect::new(
            64 * _struct[i].file + 77,
            rank_to_y(_struct[i].rank) + 6,
            75,
            75,
        );
        let src = Rect::new(527, 105, 120, 120);

        canvas.copy(&texture, src, target).unwrap();
    }
}
