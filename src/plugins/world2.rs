use bevy::prelude::*;
use blenvy::*;
use crate::prelude::plugins::game::GameState;
use crate::prelude::entity_despawner;

#[derive(Component)]
struct World2;

pub(crate) fn world2_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::LoadWorld2), setup)
        .add_systems(OnExit(GameState::InGame), entity_despawner::<World2>)
    ;
}

pub fn load_finished(
    mut blueprint_event: EventReader<BlueprintEvent>,
    mut next_state: ResMut<NextState<GameState>>,
){
    for event in blueprint_event.read(){
        if let BlueprintEvent::InstanceReady { entity: _, blueprint_name: _, blueprint_path: _ } = event {
            next_state.set(GameState::InGame);
        }    
    }
}

fn setup(mut commands: Commands) {
    info!("Loading world 2");
    commands.spawn((World2,
                    BlueprintInfo::from_path("levels/World2.glb"),
                    SpawnBlueprint,
    ));
}
