mod bar;
mod easing;
mod floating_text;
mod menu;
mod target_indicator;

use crate::{CAMERA_SCALE, WINDOW_HEIGHT, WINDOW_WIDTH};
use bar::BarPlugin;
use bevy::app::{PluginGroup, PluginGroupBuilder};
use floating_text::FloatingTextPlugin;
use menu::MenuPlugin;
use target_indicator::TargetIndicatorPlugin;

const WIDTH: f32 = WINDOW_WIDTH * CAMERA_SCALE;
const HEIGHT: f32 = WINDOW_HEIGHT * CAMERA_SCALE;

const TRANSLATION_Z: f32 = 50.0;

pub struct InterfacePlugins;

impl PluginGroup for InterfacePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(BarPlugin)
            .add(FloatingTextPlugin)
            .add(MenuPlugin)
            .add(TargetIndicatorPlugin)
    }
}
