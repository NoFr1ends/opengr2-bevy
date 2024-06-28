//
mod loader;

//
use crate::loader::GrannyLoader;

//
use bevy::app::{App, Plugin};
use bevy::asset::{Asset, AssetApp, Handle};
use bevy::reflect::TypePath;
use bevy::scene::Scene;

#[derive(Debug, Asset, TypePath)]
pub struct Granny {
    pub default_scene: Option<Handle<Scene>>,
    pub scenes: Vec<Handle<Scene>>,
}

#[derive(Default)]
pub struct GrannyPlugin;

impl Plugin for GrannyPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<GrannyLoader>()
            .init_asset::<Granny>();
    }

    fn ready(&self, _app: &App) -> bool {
        true
    }

    fn finish(&self, _app: &mut App) {
        // do nothing
    }

    fn cleanup(&self, _app: &mut App) {
        // do nothing
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    fn is_unique(&self) -> bool {
        true
    }
}
