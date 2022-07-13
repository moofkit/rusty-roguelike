use crate::prelude::*;

const MAX_ELEMENTS_COUNT: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
const RESERVE_ELEMENTS_COUNT: usize = 1000;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);
    <(&Point, &Render)>::query().iter(ecs).for_each(|(pos, render)|{
        draw_batch.set(
            *pos - offset,
            render.color,
            render.glyph
        );
    });
    draw_batch.submit(MAX_ELEMENTS_COUNT + RESERVE_ELEMENTS_COUNT).expect("Batch error");
}
