use bevy::prelude::*;

use crate::quic::{self, Payload};

pub(crate) struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup)
            .add_system_to_stage(CoreStage::First, client_connected)
            .add_system_to_stage(CoreStage::First, client_disconnected)
            .add_system_to_stage(CoreStage::First, server_connected)
            .add_system_to_stage(CoreStage::First, server_disconnected);
    }
}

#[derive(Component)]
pub(crate) struct Client;

#[derive(Component)]
pub(crate) struct Server;

#[derive(Component)]
pub(crate) struct Connection {
    pub(crate) sender: tokio::sync::mpsc::UnboundedSender<Payload>,
}

#[derive(Component)]
pub(crate) struct ConnectionId {
    pub(crate) connection_id: usize,
}

/// Handles when client connects.
fn client_connected(mut commands: Commands, mut reader: EventReader<quic::Data>) {
    for event in reader.iter() {
        let sender = match &event.payload {
            quic::Payload::ClientConnected { sender } => sender.clone(),
            _ => continue,
        };

        let span = info_span!("connection", connection_id = event.connection_id);
        let _guard = span.enter();

        let entity = commands
            .spawn()
            .insert(Name::new("client"))
            .insert(Client)
            .insert(Connection { sender })
            .insert(ConnectionId {
                connection_id: event.connection_id,
            })
            .id();

        let span = info_span!("entity", entity = ?entity);
        let _guard = span.enter();

        info!("client connected");
    }
}

/// Handles when client disconnects.
fn client_disconnected(
    mut commands: Commands,
    mut reader: EventReader<quic::Data>,
    clients: Query<(Entity, &Client, &ConnectionId)>,
) {
    for event in reader.iter() {
        match event.payload {
            quic::Payload::ClientDisconnected => (),
            _ => continue,
        }

        let span = info_span!("connection", connection_id = event.connection_id);
        let _guard = span.enter();

        let clients = clients
            .iter()
            .filter(|(_, _, c)| event.connection_id == c.connection_id);

        clients.for_each(|(entity, _, _)| {
            let span = info_span!("entity", entity = ?entity);
            let _guard = span.enter();

            commands.entity(entity).despawn();

            info!("client disconnected");
        });
    }
}

/// Handles when client connects.
fn server_connected(mut commands: Commands, mut reader: EventReader<quic::Data>) {
    for event in reader.iter() {
        let sender = match &event.payload {
            quic::Payload::ServerConnected { sender } => sender.clone(),
            _ => continue,
        };

        let span = info_span!("connection", connection_id = event.connection_id);
        let _guard = span.enter();

        let entity = commands
            .spawn()
            .insert(Name::new("server"))
            .insert(Server)
            .insert(Connection { sender })
            .insert(ConnectionId {
                connection_id: event.connection_id,
            })
            .id();

        let span = info_span!("entity", entity = ?entity);
        let _guard = span.enter();

        info!("server connected");
    }
}

/// Handles when client disconnects.
fn server_disconnected(
    mut commands: Commands,
    mut reader: EventReader<quic::Data>,
    servers: Query<(Entity, &Server, &ConnectionId)>,
) {
    for event in reader.iter() {
        match event.payload {
            quic::Payload::ServerDisconnected => (),
            _ => continue,
        }

        let span = info_span!("connection", connection_id = event.connection_id);
        let _guard = span.enter();

        let clients = servers
            .iter()
            .filter(|(_, _, c)| event.connection_id == c.connection_id);

        clients.for_each(|(entity, _, _)| {
            let span = info_span!("entity", entity = ?entity);
            let _guard = span.enter();

            commands.entity(entity).despawn();

            info!("server disconnected");
        });
    }
}

/// TODO: triggers connection command to server
fn startup(control: ResMut<tokio::sync::mpsc::UnboundedSender<quic::Control>>) {
    if cfg!(feature = "client") {
        if let Err(error) = control.send(quic::Control::Connect {
            address: "172.0.0.1".to_string(),
            port: 4433,
        }) {
            error!(error = ?error, "Failed to send connect");
        }
    }
}
