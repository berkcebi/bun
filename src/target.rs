use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Target {
    pub entity: Option<Entity>,
}