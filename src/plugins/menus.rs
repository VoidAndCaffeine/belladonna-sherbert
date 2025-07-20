use bevy::prelude::*;
use crate::prelude::game::GameState;

//Todo:
// non-startup loading screens
// support variable load time
// coffee constellations logo
pub(super) mod loading {
    use bevy::prelude::*;
    use super::{despawn_screen, GameState};

    pub fn loading_plugin(app: &mut App) {
        app
            .add_systems(OnEnter(GameState::LoadingScreen), loading_setup)
            .add_systems(Update, countdown.run_if(in_state(GameState::LoadingScreen)))
            .add_systems(OnExit(GameState::LoadingScreen), despawn_screen::<OnSplashScreen>);
    }

    // tag for splash screen objects, makes deconstruction easy
    #[derive(Component)]
    struct OnSplashScreen;

    #[derive(Resource, Deref, DerefMut)]
    struct SplashTimer(Timer);

    fn loading_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
}
pub(super) mod main_menu {
    use bevy::{
        app::AppExit,
        color::palettes::css::BLACK,
        ecs::spawn::{SpawnIter, SpawnWith},
        prelude::*,
    };
    use bevy::asset::meta::Settings;
    use bevy::audio::Volume;
    use bevy::text::cosmic_text::ttf_parser::Weight::Black;
    use crate::prelude::fonts::FontAssets;
    use super::{despawn_screen, GameState};


    // Button Colors
    const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
    const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
    const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
    const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
    const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

    // components
    #[derive(Component)]
    struct SelectedOption;


    pub fn main_menu_plugin(app: &mut App) {
        app
            .add_systems(OnEnter(GameState::MainMenu), main_menu_setup)
            .add_systems(OnExit(GameState::MainMenu), despawn_screen::<OnMainMenu>)
            .add_systems(Update, button_system.run_if(in_state(GameState::MainMenu)))
        ;
    }

    // button setup
    fn button_system(
        mut interaction_query: Query<
            (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
            (Changed<Interaction>, With<Button>),
        >,
    ) {
        for (interaction, mut background_color, selected) in &mut interaction_query {
            *background_color = match (*interaction, selected) {
                (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
                (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
                (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
                (Interaction::None, None) => NORMAL_BUTTON.into(),
            }
        }
    }

    // tag for splash screen objects, makes deconstruction easy
    #[derive(Component)]
    struct OnMainMenu;

    fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>, fonts: Res<FontAssets>) {

        let button_node = Node {
            width: Val::Px(300.0),
            height: Val::Px(65.0),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };

        let button_icon_node = Node {
            width: Val::Px(30.0),
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            ..default()
        };

        let button_text_font = TextFont {
            font_size: 33.0,
            ..default()
        };

        //let bird_icon = asset_server.load("textures/bevy.png");
        let right_icon = asset_server.load("textures/buttons/right.png");
        let wrench_icon = asset_server.load("textures/buttons/wrench.png");
        let exit_icon = asset_server.load("textures/buttons/exitRight.png");
        commands.spawn((
            Node {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            OnMainMenu,
            children![(
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems:: Center,
                    ..default()
                },
                BackgroundColor(BLACK.into()),
                children![
                    (
                        Text::new("Belladonna-sherbet"),
                            TextFont {
                            font_size: 67.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                        Node {
                            margin: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                    ),
                    (
                        Button,
                        button_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        children![
                            (ImageNode::new(right_icon.clone()), button_icon_node.clone()),
                            (
                                Text::new("New Game"),
                                button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ),
                        ]
                    ),
                    (
                        Button,
                        button_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        children![
                            (ImageNode::new(right_icon), button_icon_node.clone()),
                            (
                                Text::new("Load Game"),
                                button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ),
                        ]
                    ),
                    (
                        Button,
                        button_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        children![
                            (ImageNode::new(wrench_icon), button_icon_node.clone()),
                            (
                                Text::new("Settings"),
                                button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ),
                        ]
                    ),
                    (
                        Button,
                        button_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        children![
                            (ImageNode::new(exit_icon), button_icon_node.clone()),
                            (
                                Text::new("Quit"),
                                button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ),
                        ]
                    ),
                ]
            )],
        ));
    }
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }
}