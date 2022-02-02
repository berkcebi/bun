mod bar;
mod easing;
mod floating_text;
mod target_indicator;

use bar::BarPlugin;
use bevy::app::{PluginGroup, PluginGroupBuilder};
use floating_text::FloatingTextPlugin;
use target_indicator::TargetIndicatorPlugin;

use crate::{CAMERA_SCALE, WINDOW_HEIGHT, WINDOW_WIDTH};

const WIDTH: f32 = WINDOW_WIDTH * CAMERA_SCALE;
const HEIGHT: f32 = WINDOW_HEIGHT * CAMERA_SCALE;

const TRANSLATION_Z: f32 = 50.0;

pub struct InterfacePlugins;

impl PluginGroup for InterfacePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(BarPlugin);
        group.add(FloatingTextPlugin);
        group.add(TargetIndicatorPlugin);
    }
}
