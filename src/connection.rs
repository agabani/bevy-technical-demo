use bevy::prelude::*;

use crate::quic;

pub(crate) struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(client_connection_lost);
    }
}

/// Handles when client connection was lost.
fn client_connection_lost(mut reader: EventReader<quic::Protocol>) {
    for event in reader.iter() {
        info!(event = ?event, "connection");
    }
}
