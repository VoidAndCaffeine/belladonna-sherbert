use bevy::prelude::*;
use blenvy::*;
use crate::prelude::plugins::game::GameState;
use crate::prelude::entity_despawner;

#[derive(Component)]
struct World1;

pub(crate) fn world1_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::InGameWorld1), setup)
        .add_systems(OnExit(GameState::InGameWorld1), entity_despawner::<World1>)
    ;
}

fn setup(mut commands: Commands) {
    info!("Loading world 1");
    commands.spawn((World1,
        BlueprintInfo::from_path("levels/World.glb"),
        SpawnBlueprint,
    ));
}