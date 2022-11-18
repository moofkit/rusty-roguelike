#![warn(clippy::pedantic)]

mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut builder = MapBuilder::new(&mut rng);
        spawn_player(&mut ecs, builder.player_start);
        let idx_exit = builder.map.point2d_to_index(builder.amulet_start);
        builder.map.tiles[idx_exit] = TileType::Exit;
        spawn_level(&mut ecs, &mut rng, 0, &builder.monster_spawns);
        resources.insert(builder.map);
        resources.insert(Camera::new(builder.player_start));
        resources.insert(TurnState::AwaitingInput);
        resources.insert(builder.theme);
        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
    }

    fn reset_game_state(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut builder = MapBuilder::new(&mut rng);
        spawn_player(&mut self.ecs, builder.player_start);
        let idx_exit = builder.map.point2d_to_index(builder.amulet_start);
        builder.map.tiles[idx_exit] = TileType::Exit;
        spawn_level(&mut self.ecs, &mut rng, 0, &builder.monster_spawns);
        self.resources.insert(builder.map);
        self.resources.insert(Camera::new(builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(builder.theme);
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, RED, BLACK, "Your quest has ended.");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "Slain by a monster, your hero's journey has come to a premature end.",
        );
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "The Amulet of Yala remains unclaimed, and your home town is not saved.",
        );
        ctx.print_color_centered(8, GREEN, BLACK, "Press 1 to play again");
        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, GREEN, BLACK, "You have won!");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "You put on the Amulet of Yala and feel its power course through your veins",
        );
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "Your town is saved and you can return to normal life",
        );
        ctx.print_color_centered(8, GREEN, BLACK, "Press 1 to play again");
        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }

    fn advance_level(&mut self) {
        use std::collections::HashSet;

        let player_entity = *<Entity>::query()
            .filter(component::<Player>())
            .iter(&mut self.ecs)
            .nth(0)
            .unwrap();
        let mut entities_to_keep = HashSet::new();
        entities_to_keep.insert(player_entity);
        <(Entity, &Carried)>::query()
            .iter(&self.ecs)
            .filter(|(_, carry)| carry.0 == player_entity)
            .map(|(entity, _)| *entity)
            .for_each(|entity| {
                entities_to_keep.insert(entity);
            });
        let mut cb = CommandBuffer::new(&self.ecs);
        for enitiy in Entity::query().iter(&self.ecs) {
            if !entities_to_keep.contains(enitiy) {
                cb.remove(*enitiy);
            }
        }
        cb.flush(&mut self.ecs);
        <&mut FieldOfView>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|fov| fov.is_dirty = true);
        let mut rng = RandomNumberGenerator::new();
        let mut builder = MapBuilder::new(&mut rng);
        let mut map_level = 0;
        <(&mut Player, &mut Point)>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|(player, pos)| {
                *pos = builder.player_start;
                player.map_level += 1;
                map_level = player.map_level;
            });
        if map_level == 2 {
            spawn_amulet(&mut self.ecs, builder.amulet_start);
        } else {
            let idx_exit = builder.map.point2d_to_index(builder.amulet_start);
            builder.map.tiles[idx_exit] = TileType::Exit;
        }
        spawn_level(
            &mut self.ecs,
            &mut rng,
            map_level as usize,
            &builder.monster_spawns,
        );
        self.resources.insert(builder.map);
        self.resources.insert(Camera::new(builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(builder.theme);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.resources.insert(ctx.key);
        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();
        let current_state = *self.resources.get::<TurnState>().unwrap();
        match current_state {
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self
                .player_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::MonsterTurn => self
                .monster_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::GameOver => self.game_over(ctx),
            TurnState::Victory => self.victory(ctx),
            TurnState::NextLevel => self.advance_level(),
        }
        render_draw_buffer(ctx).expect("Render error");
    }
}

embedded_resource!(TILE_FONT, "../resources/dungeonfont.png");

fn main() -> BError {
    link_resource!(TILE_FONT, "resources/dungeonfont.png");
    let context = BTermBuilder::new()
        .with_title("Rusty Roguelike")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, "terminal8x8.png")
        .build()?;
    main_loop(context, State::new())
}
