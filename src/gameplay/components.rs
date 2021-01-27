use bevy::prelude::*;

pub struct StaffRotator;
pub struct Enemy;
pub struct Ineffective(pub Timer);
#[derive(Default)]
pub struct Velocity(pub Vec2);
pub struct Player;
pub struct TargetCircle;
pub struct Background;
#[derive(Default)]
pub struct Damage(pub usize);
