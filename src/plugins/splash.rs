//Todo:
// non-startup loading screens
// support variable load time
// coffee constellations logo
use crate::prelude::game::{GameState,LoadingState};
use crate::prelude::plugins::ui::despawn_screen;
use bevy::prelude::*;

pub(crate) fn splash_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::LoadingScreen),
                     loading_main_menu.run_if(in_state(LoadingState::MainMenu)))
        .add_systems(Update, countdown.run_if(in_state(LoadingState::MainMenu)))
        .add_systems(Update, countdown.run_if(in_state(LoadingState::MainMenuSilent)))
        .add_systems(
            OnExit(GameState::LoadingScreen),
            despawn_screen::<OnSplashScreen>,
        );
}

// tag for splash screen objects, makes deconstruction easy
#[derive(Component)]
struct OnSplashScreen;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn loading_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("textures/bevy.png");
    // birb.
    commands.spawn((
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        OnSplashScreen,
        // child of ^ node
        children![(
            ImageNode::new(icon),
            Node {
                width: Val::Px(200.0),
                ..default()
            },
        )],
    ));
    // insert timer resource into engine
    commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
}


fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::MainMenu);
    }
}
