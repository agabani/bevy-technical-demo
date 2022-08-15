use bevy::prelude::*;

/// Runs application.
///
/// # Errors
///
/// Returns error if application is unable to start.
pub fn run() -> crate::Result<()> {
    let mut app = App::new();
    let runtime = tokio::runtime::Runtime::new()?;

    add_backends(&mut app, &runtime);
    add_default_plugins(&mut app);
    add_game_plugins(&mut app);

    app.run();

    Ok(())
}

/// Adds backends.
fn add_backends(app: &mut App, runtime: &tokio::runtime::Runtime) {
    use crate::{postgres, quic};

    postgres::backend(app, runtime);
    quic::backend(app, runtime);
}

/// Adds default plugins.
fn add_default_plugins(app: &mut App) {
    if cfg!(feature = "headless") {
        app.add_plugin(bevy::log::LogPlugin::default())
            .add_plugins(MinimalPlugins);
    } else {
        app.add_plugins(DefaultPlugins);
    }
}

/// Adds game plugins.
fn add_game_plugins(app: &mut App) {
    use crate::{character, connection, postgres, quic};

    app.add_plugin(character::Plugin)
        .add_plugin(connection::Plugin)
        .add_plugin(postgres::Plugin)
        .add_plugin(quic::Plugin);
}
