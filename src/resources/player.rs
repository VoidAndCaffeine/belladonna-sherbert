use avian3d::prelude::{Collider, LockedAxes, RigidBody};
use bevy::color::palettes::css;
use bevy::prelude::*;
use bevy_tnua::prelude::{TnuaBuiltinJump, TnuaBuiltinWalk, TnuaController};
use bevy_tnua::TnuaUserControlsSystemSet;
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use leafwing_input_manager::prelude::{ActionState, InputMap};
use crate::plugins::game::GameState;
use crate::plugins::input::PlayerAction;
use crate::prelude::input;

#[derive(Component,Reflect)]
#[reflect(Component)]
pub struct Player;

pub fn player_plugin(app: &mut App) {
    app
        .register_type::<Player>()
        .add_systems(
            FixedUpdate,
            apply_controls.in_set(TnuaUserControlsSystemSet)
        )
    ;
}

fn apply_controls(
    time: Res<Time>,
    action_state: Res<ActionState<PlayerAction>>,
    mut query: Query<&mut TnuaController>
){
    let Ok(mut controller) = query.single_mut() else { return; };
    let mut direction = Vec3::ZERO;
    if action_state.axis_pair(&PlayerAction::Move) != Vec2::ZERO{
        let move_delta =
            time.delta_secs() * action_state.clamped_axis_pair(&PlayerAction::Move);
        direction.x += move_delta.y;
        direction.z += move_delta.x;
    }

    // Feed the basis every frame. Even if the player doesn't move - just use `desired_velocity:
    // Vec3::ZERO`. `TnuaController` starts without a basis, which will make the character collider
    // just fall.
    controller.basis(TnuaBuiltinWalk {
        // The `desired_velocity` determines how the character will move.
        desired_velocity: direction.normalize_or_zero() * 10.0,
        // The `float_height` must be greater (even if by little) from the distance between the
        // character's center and the lowest point of its collider.
        float_height: 1.0,
        // `TnuaBuiltinWalk` has many other fields for customizing the movement - but they have
        // sensible defaults. Refer to the `TnuaBuiltinWalk`'s documentation to learn what they do.
        ..Default::default()
    });

    // Feed the jump action every frame as long as the player holds the jump button. If the player
    // stops holding the jump button, simply stop feeding the action.
    if action_state.just_pressed(&PlayerAction::Jump) {
        controller.action(TnuaBuiltinJump {
            // The height is the only mandatory field of the jump button.
            height: 4.0,
            // `TnuaBuiltinJump` also has customization fields with sensible defaults.
            ..Default::default()
        });
    }
}
