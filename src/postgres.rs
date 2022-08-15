use bevy::prelude::*;

use crate::quic;

#[derive(Debug, Clone)]
pub(crate) enum Request {
    ClientAuthenticate(ClientAuthenticate),
}

#[derive(Debug, Clone)]
pub(crate) enum Response {
    ClientAuthenticated(ClientAuthenticated),
}

#[derive(Debug, Clone)]
pub(crate) struct ClientAuthenticate {
    pub(crate) connection_id: usize,
    pub(crate) username: String,
    pub(crate) password: String,
}

#[derive(Debug, Clone)]
pub(crate) struct ClientAuthenticated {
    pub(crate) connection_id: usize,
    pub(crate) username: String,
}

pub(crate) fn backend(app: &mut App, runtime: &tokio::runtime::Runtime) {
    let (request_sender, request_receiver) = tokio::sync::mpsc::unbounded_channel();
    let (response_sender, response_receiver) = tokio::sync::mpsc::unbounded_channel();

    async fn handle(
        mut request_receiver: tokio::sync::mpsc::UnboundedReceiver<Request>,
        response_sender: tokio::sync::mpsc::UnboundedSender<Response>,
    ) -> crate::Result<()> {
        while let Some(request) = request_receiver.recv().await {
            match request {
                Request::ClientAuthenticate(client_authenticate) => {
                    if client_authenticate.username == "foo"
                        && client_authenticate.password == "bar"
                    {
                        response_sender.send(Response::ClientAuthenticated(
                            ClientAuthenticated {
                                connection_id: client_authenticate.connection_id,
                                username: client_authenticate.username,
                            },
                        ))?
                    }
                }
            }
        }

        Ok(())
    }

    app.insert_resource(request_sender);
    app.insert_resource(response_receiver);

    runtime.spawn(handle(request_receiver, response_sender));
}

pub(crate) struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Response>()
            .add_system_to_stage(CoreStage::First, responses);
    }
}

/// Translates inbound response from tokio runtime to bevy runtime.
fn responses(
    mut writer: EventWriter<Response>,
    mut response_receiver: ResMut<tokio::sync::mpsc::UnboundedReceiver<Response>>,
) {
    while let Ok(response) = response_receiver.try_recv() {
        writer.send(response);
    }
}
