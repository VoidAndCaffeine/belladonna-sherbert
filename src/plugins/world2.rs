use avian3d::prelude::{Collider, LockedAxes, RigidBody};
use bevy::color::palettes::css;
use bevy::prelude::*;
use bevy_tnua::prelude::TnuaController;
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use blenvy::*;
use crate::prelude::plugins::game::GameState;
use crate::prelude::entity_despawner;
use crate::prelude::player::Player;

#[derive(Component)]
struct World2;

pub(crate) fn world2_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::LoadWorld2), (setup_level, setup_player))
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

fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("Creating player");
    commands.spawn((
        Player,
        Mesh3d(meshes.add(Capsule3d{
            radius: 0.5,
            half_length: 0.5,
        })),
        MeshMaterial3d(materials.add(Color::from(css::DARK_CYAN))),
        Transform::from_xyz(0.0, 0.1, 0.0),
        RigidBody::Dynamic,
        Collider::capsule(0.5,1.0),
        TnuaController::default(),
        TnuaAvian3dSensorShape(Collider::cylinder(0.49,0.0)),
        LockedAxes::ROTATION_LOCKED,
        children![(
        Camera3d::default(),
        Transform::from_xyz(-10.0, 2.0, 0.0).looking_at(Vec3::new(0.0,0.0,0.0), Vec3::Y),
        )]
    ));
}


fn setup_level(mut commands: Commands) {
    info!("Loading world 2");
    commands.spawn((World2,
                    BlueprintInfo::from_path("levels/World2.glb"),
                    SpawnBlueprint,
    ));
}
