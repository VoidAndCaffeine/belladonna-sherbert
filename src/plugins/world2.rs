use bevy::prelude::*;
use blenvy::*;
use crate::prelude::plugins::game::GameState;
use crate::prelude::entity_despawner;

#[derive(Component)]
struct World2;

pub(crate) fn world2_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::InGameWorld2), setup)
        .add_systems(OnExit(GameState::InGameWorld2), entity_despawner::<World2>)
    ;
}

fn setup(mut commands: Commands) {
    info!("Loading world 2");
    commands.spawn((World2,
                    BlueprintInfo::from_path("levels/World2.glb"),
                    SpawnBlueprint,
    ));
}
