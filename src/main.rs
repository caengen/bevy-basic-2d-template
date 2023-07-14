use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    diagnostic::FrameTimeDiagnosticsPlugin,
    log::{Level, LogPlugin},
    prelude::*,
    window::PresentMode,
    DefaultPlugins,
};
use bevy_asset_loader::prelude::{AssetCollection, LoadingState, LoadingStateAppExt};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_turborand::prelude::RngPlugin;
use config::Debug;
use game::GamePlugin;
use main_menu::*;
use std::{env, process};

mod config;
mod game;
mod main_menu;

pub const SCREEN: Vec2 = Vec2::from_array([495.0, 270.0]);
pub const DARK: Color = Color::rgb(0.191, 0.184, 0.156);
pub const LIGHT: Color = Color::rgb(0.852, 0.844, 0.816);

// Example: Easy loading of assets
// #[derive(AssetCollection, Resource)]
// pub struct ImageAssets {
//     #[asset(texture_atlas(tile_size_x = 16.0, tile_size_y = 16.0, columns = 8, rows = 1))]
//     #[asset(path = "textures/images.png")]
//     pub images: Handle<TextureAtlas>,
// }

#[derive(States, Hash, Clone, PartialEq, Eq, Debug, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
}

/**
 * The configuration for the game loop. For cleanliness
 */
fn main() {
    // Possibility for program args
    let args: Vec<String> = env::args().skip(1).collect();
    let cfg = config::ProgramConfig::build(&args).unwrap_or_else(|err| {
        println!("A problem occured when parsing args: {err}");
        process::exit(1);
    });

    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "TITLE OF YOUR GAME".into(),
                    resolution: (SCREEN.x, SCREEN.y).into(),
                    present_mode: PresentMode::AutoNoVsync,
                    // Tells wasm to resize the window according to the available canvas
                    fit_canvas_to_parent: true,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            })
            .set(LogPlugin {
                level: Level::DEBUG,
                filter: "wgpu=error,bevy_render=info,bevy_ecs=trace".to_string(),
            })
            .set(ImagePlugin::default_nearest()),
    )
    .add_state::<GameState>()
    .insert_resource(Debug(cfg.debug))
    // Example: Easy loading of assets
    // .add_collection_to_loading_state::<_, ImageAssets>(GameState::AssetLoading)
    .add_plugins(FrameTimeDiagnosticsPlugin::default())
    // .add_plugins(
    //     WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
    // )
    .add_plugins(RngPlugin::new().with_rng_seed(220718))
    .add_plugins(MainMenuPlugin)
    .add_plugins(GamePlugin)
    .add_systems(Startup, setup);

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(DARK),
        },
        ..default()
    });
}
