use avian3d::prelude::*;
use bevy::{asset::{AssetMetaCheck,load_internal_binary_asset}, prelude::*};
use bevy::render::RenderSet::PhaseSort;
use bevy::window::PresentMode;
use blenvy::*;

const BACKGROUND_COLOR: Color = Color::srgb(1.0, 0.0, 1.0);

// Sets up the default plugins like windows, assets, etc

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch. You can enable this
                    // if you want to use meta files and are not building for the web
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Belladonna Sherbet".into(),
                        resizable: false,
                        resolution: (1920., 1080.).into(),
                        present_mode: PresentMode::Immediate,
                        canvas: Some("#bevy".to_owned()),
                        desired_maximum_frame_latency: core::num::NonZero::new(1u32),
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                }),
            BlenvyPlugin::default(),
            PhysicsPlugins::default(),
        ))
    ;

    // set default font to Rotis
    load_internal_binary_asset!(
        app,
        TextFont::default().font,
        "../../assets/fonts/Rotis Serif Std/Rotis Serif Std 55 Regular/Rotis Serif Std 55 Regular.otf",
        |bytes: &[u8], _path: String| {Font::try_from_bytes(bytes.to_vec()).unwrap()});
}
