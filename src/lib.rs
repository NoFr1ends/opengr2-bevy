mod loader;

use bevy::app::{App, Plugin};
use bevy::asset::{AddAsset, Handle};
use bevy::reflect::TypeUuid;
use bevy::scene::Scene;
use crate::loader::GrannyLoader;

#[derive(Debug, TypeUuid)]
#[uuid = "750b0677-5e1f-405b-8680-9910ccdc7809"]
pub struct Granny {
    pub default_scene: Option<Handle<Scene>>,
    pub scenes: Vec<Handle<Scene>>
}

#[derive(Default)]
pub struct GrannyPlugin {}

impl Plugin for GrannyPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<GrannyLoader>()
            .add_asset::<Granny>();
    }
}