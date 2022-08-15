use bevy::prelude::*;

use crate::{connection, postgres, quic};

pub(crate) struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(client_authenticate)
            .add_system(client_authenticate_validated)
            .add_system(client_authenticated);
    }
}

#[derive(Component)]
pub(crate) struct Authenticated {
    username: String,
}

/// Handles client authenticated request
fn client_authenticate(
    mut reader: EventReader<quic::Data>,
    database: Res<tokio::sync::mpsc::UnboundedSender<postgres::Request>>,
) {
    for data in reader.iter() {
        let client_authenticate = match &data.payload {
            quic::Payload::ClientAuthenticate(client_authenticate) => client_authenticate,
            _ => continue,
        };

        let span = info_span!("connection", connection_id = data.connection_id);
        let _guard = span.enter();

        let span = info_span!("account", username = client_authenticate.username);
        let _guard = span.enter();

        if let Err(error) = database.send(postgres::Request::ClientAuthenticate(
            postgres::ClientAuthenticate {
                connection_id: data.connection_id,
                username: client_authenticate.username.clone(),
                password: client_authenticate.password.clone(),
            },
        )) {
            error!(error = ?error, "Failed");
        } else {
            info!("Sent");
        }
    }
}

fn client_authenticate_validated(
    mut responses: EventReader<postgres::Response>,
    connections: Query<(
        &connection::Client,
        &connection::Connection,
        &connection::ConnectionId,
    )>,
    mut writer: EventWriter<quic::Data>,
) {
    for response in responses.iter() {
        let response = match response {
            postgres::Response::ClientAuthenticated(client_authenticated) => client_authenticated,
        };

        let span = info_span!("connection", connection_id = response.connection_id);
        let _guard = span.enter();

        let span = info_span!("account", username = response.username);
        let _guard = span.enter();

        info!("Authenticate validated");

        let payload = quic::Payload::ClientAuthenticated {
            username: response.username.clone(),
        };

        connections
            .iter()
            .filter(|(_, _, connection_id)| response.connection_id == connection_id.connection_id)
            .for_each(|(_, connection, _)| {
                if let Err(error) = connection.sender.send(payload.clone()) {
                    error!(error = ?error,"Failed to send");
                }
            });

        writer.send(quic::Data {
            connection_id: response.connection_id,
            payload,
        });
    }
}

fn client_authenticated(
    mut commands: Commands,
    mut reader: EventReader<quic::Data>,
    connections: Query<(
        Entity,
        &connection::Client,
        &connection::Connection,
        &connection::ConnectionId,
    )>,
) {
    for data in reader.iter() {
        let username = match &data.payload {
            quic::Payload::ClientAuthenticated { username } => username,
            _ => continue,
        };

        let span = info_span!("connection", connection_id = data.connection_id);
        let _guard = span.enter();

        let span = info_span!("account", username = username);
        let _guard = span.enter();

        connections
            .iter()
            .filter(|(_, _, _, connection_id)| data.connection_id == connection_id.connection_id)
            .for_each(|(entity, _, _, _)| {
                commands.entity(entity).insert(Authenticated {
                    username: username.clone(),
                });

                info!("Authenticated");
            });
    }
}
