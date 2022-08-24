use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantToMove {
    pub entity: Entity,
    pub destination: Point,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Name(pub String);
