use bevy::prelude::*;

pub struct GameplayEntity;

pub struct StaffRotator;
pub struct Enemy;
pub struct Ineffective(pub Timer);
#[derive(Default)]
pub struct Velocity(pub Vec2);
pub struct Player;
pub struct TargetCircle;
#[derive(Default)]
pub struct Damage(pub usize);
