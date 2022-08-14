use bevy::prelude::*;

#[derive(Debug, Clone)]
pub(crate) enum Payload {
    ClientConnected {
        sender: tokio::sync::mpsc::UnboundedSender<Payload>,
    },
    ClientDisconnected,
}

#[derive(Debug, Clone)]
pub(crate) struct Inbound {
    pub(crate) connection_id: usize,
    pub(crate) payload: Payload,
}

pub(crate) fn backend(app: &mut App, runtime: &tokio::runtime::Runtime) {
    let (inbound_sender, inbound_receiver) = tokio::sync::mpsc::unbounded_channel();

    async fn handle_inbound(
        inbound_sender: tokio::sync::mpsc::UnboundedSender<Inbound>,
    ) -> crate::Result<()> {
        let mut connection_id_connect = 0;
        let mut connection_id_disconnect = 0;

        loop {
            let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel();

            connection_id_connect += 1;
            inbound_sender.send(Inbound {
                connection_id: connection_id_connect,
                payload: Payload::ClientConnected { sender },
            })?;

            if connection_id_connect % 2 == 0 {
                connection_id_disconnect += 1;
                inbound_sender.send(Inbound {
                    connection_id: connection_id_disconnect,
                    payload: Payload::ClientDisconnected,
                })?;
            }

            tokio::spawn(async move {
                while let Some(payload) = receiver.recv().await {
                    println!("{:?}", payload);
                }
            });

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }

    app.insert_resource(inbound_receiver);

    runtime.spawn(handle_inbound(inbound_sender));
}

pub(crate) struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Inbound>()
            .add_system_to_stage(CoreStage::First, inbound);
    }
}

/// Translates inbound messages from tokio runtime to bevy runtime.
fn inbound(
    mut writer: EventWriter<Inbound>,
    mut inbound_receiver: ResMut<tokio::sync::mpsc::UnboundedReceiver<Inbound>>,
) {
    while let Ok(inbound) = inbound_receiver.try_recv() {
        writer.send(inbound);
    }
}
