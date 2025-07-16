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
    use super::{despawn_screen, GameState};
    use crate::prelude::resources::settings::*;

    pub fn main_menu_plugin(app: &mut App) {
        app
            .init_state::<MenuState>()
            .add_systems(OnEnter(GameState::MainMenu), menu_setup)
            // main menu
            .add_systems(OnEnter(MenuState::Main), main_menu_setup)
            .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainScreen>)
            // settings
            .add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
            .add_systems(OnExit(MenuState::Settings), despawn_screen::<OnSettingsScreen>)
            // display settings
            .add_systems(OnEnter(MenuState::SettingsDisplay), display_settings_menu_setup)
            .add_systems(Update,setting_button::<DisplayQuality>
                .run_if(in_state(MenuState::SettingsDisplay)))
            .add_systems(OnExit(MenuState::SettingsDisplay),
                         despawn_screen::<OnSettingsDisplayScreen>)
            // sound settings
            .add_systems(OnEnter(MenuState::SettingsSound), sound_settings_menu_setup)
            .add_systems(Update, setting_button::<VolumeSetting>
                .run_if(in_state(MenuState::SettingsSound)))
            .add_systems(OnExit(MenuState::SettingsSound),despawn_screen::<OnSettingsSoundScreen>)
            // common
            .add_systems(Update, (menu_action, button_system)
                .run_if(in_state(GameState::MainMenu)));
    }

    #[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
    enum MenuState {
        Main,
        Settings,
        SettingsDisplay,
        SettingsSound,
        #[default]
        Disabled,
    }
    #[derive(Component)]
    struct OnMainScreen;
    #[derive(Component)]
    struct OnSettingsScreen;
    #[derive(Component)]
    struct OnSettingsDisplayScreen;
    #[derive(Component)]
    struct OnSettingsSoundScreen;


    const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

    const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
    const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
    const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
    const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

    #[derive(Component)]
    struct SelectedOption;

    #[derive(Component)]
    enum MenuButtonAction {
        Play,
        Settings,
        SettingsDisplay,
        SettingsSound,
        BackToMain,
        BackToSettings,
        Quit,
    }

    // button color changing
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

    fn setting_button<T: Resource + Component + PartialEq + Copy>(
        interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
        selected_query: Single<(Entity, &mut BackgroundColor), With<SelectedOption>>,
        mut commands: Commands,
        mut setting: ResMut<T>,
    ) {
        let (previous_button, mut previous_button_color) = selected_query.into_inner();
        for(interaction, button_setting, entity) in &interaction_query {
            if *interaction == Interaction::Pressed && *setting != *button_setting {
                *previous_button_color = NORMAL_BUTTON.into();
                commands.entity(previous_button).remove::<SelectedOption>();
                commands.entity(entity).insert(SelectedOption);
                *setting = *button_setting;
            }
        }
    }

    fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
        menu_state.set(MenuState::Main);
    }

    fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

        let right_icon = asset_server.load("textures/buttons/right.png");
        let wrench_icon = asset_server.load("textures/buttons/wrench.png");
        let exit_icon = asset_server.load("textures/buttons/exitRight.png");

        commands.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnMainScreen,
            children![
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(BLACK.into()),
                children![(
                    Text::new("Menu Test Text"),
                    TextFont { font_size: 67.0, ..default()},
                    TextColor(TEXT_COLOR),
                    Node {margin: UiRect::all(Val::Px(50.0)),..default()},
                    ), (
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Play,
                    children![
                        (ImageNode::new(right_icon), button_icon_node.clone()),
                        (
                            Text::new("New Game"),
                            button_text_font.clone(),
                            TextColor(TEXT_COLOR),
                        ),
                    ]
                    ), (
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Settings,
                    children![
                        (ImageNode::new(wrench_icon), button_icon_node.clone()),
                        (
                            Text::new("New Game"),
                            button_text_font.clone(),
                            TextColor(TEXT_COLOR),
                        ),
                    ]
                    ), (
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Quit,
                    children![
                        (ImageNode::new(exit_icon), button_icon_node.clone()),
                        (
                            Text::new("New Game"),
                            button_text_font.clone(),
                            TextColor(TEXT_COLOR),
                        ),
                    ]
                )]
            ]
        ));
    }

    fn settings_menu_setup(mut commands: Commands){
        let button_node = Node {
            width: Val::Px(200.0),
            height: Val::Px(65.0),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };

        let button_text_style = (
            TextFont { font_size: 33.0, ..default()},
            TextColor(TEXT_COLOR),
        );

        commands.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnSettingsScreen,
            children![(
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(BLACK.into()),
                Children::spawn(SpawnIter(
                    [
                        (MenuButtonAction::SettingsDisplay, "Display"),
                        (MenuButtonAction::SettingsSound, "Sound"),
                        (MenuButtonAction::BackToMain, "Back"),
                    ].into_iter().map(move|(action,text)| {
                        (
                            Button,
                            button_node.clone(),
                            BackgroundColor(NORMAL_BUTTON),
                            action,
                            children![(Text::new(text), button_text_style.clone())]
                        )
                    })
                ))
            )]
        ));
    }

    fn display_settings_menu_setup(mut commands: Commands, display_quality: Res<DisplayQuality>) {}
    fn sound_settings_menu_setup(mut commands: Commands, volume: Res<VolumeSetting>) {}
    fn menu_action(
        interaction_query: Query<
            (&Interaction, &MenuButtonAction),
            (Changed<Interaction>, With<Button>),
        >,
        mut app_exit_events: EventWriter<AppExit>,
        mut menu_state: ResMut<NextState<MenuState>>,
        mut game_state: ResMut<NextState<GameState>>,
    ){
        for (interaction, menu_button_action) in &interaction_query{
            if *interaction == Interaction::Pressed{
                match menu_button_action {
                    MenuButtonAction::Quit => {app_exit_events.write(AppExit::Success); }
                    MenuButtonAction::Play => {
                        menu_state.set(MenuState::Disabled);
                        game_state.set(GameState::InGame);
                    }
                    MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
                    MenuButtonAction::SettingsDisplay => menu_state.set(MenuState::SettingsDisplay),
                    MenuButtonAction::SettingsSound => menu_state.set(MenuState::SettingsSound),
                    MenuButtonAction::BackToMain => menu_state.set(MenuState::Main),
                    MenuButtonAction::BackToSettings => menu_state.set(MenuState::Settings),
                }
            }
        }
    }
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }
}