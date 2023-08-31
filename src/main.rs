use bevy::prelude::*;

mod player;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window{
                        title: "Wizard Game".into(),
                        resolution: (640.0,480.0).into(),
                        resizable:false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
            player::PlayerPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}
fn setup(mut commands:Commands){
    commands.spawn(Camera2dBundle::default());
}


