use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push(
        (
            Player,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('@'),
            }
        )
    );
}

pub fn spawn_enemy(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    ecs.push(
        (
            Enemy,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: match rng.range(0, 4) {
                    0 => to_cp437('E'), // ettin
                    1 => to_cp437('O'), // orc
                    2 => to_cp437('o'), // ogr
                    _ => to_cp437('g') // goblin
                }
            },
            MovingRandomly
        )
    );
}
