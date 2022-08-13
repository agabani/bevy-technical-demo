use bevy::prelude::*;

/// Runs application.
///
/// # Errors
///
/// Returns error if application is unable to start
pub fn run() -> crate::Result<()> {
    let mut app = App::new();

    add_default_plugins(&mut app);

    app.run();

    Ok(())
}

/// Adds default plugins.
fn add_default_plugins(app: &mut App) {
    if cfg!(feature = "client") {
        app.add_plugins(DefaultPlugins);
    }

    if cfg!(feature = "server") {
        app.add_plugin(bevy::log::LogPlugin::default())
            .add_plugins(MinimalPlugins);
    }
}
