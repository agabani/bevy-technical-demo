use bevy::prelude::*;

/// Controls the quic module
#[derive(Debug, Clone)]
pub enum Control {
    Connect { address: String, port: u16 },
    Disconnect,
}

/// Incoming data from peer
#[derive(Debug, Clone)]
pub(crate) struct Data {
    /// Quic Connection ID of peer
    pub(crate) connection_id: usize,
    pub(crate) payload: Payload,
}

/// Payload sent between peers
#[derive(Debug, Clone)]
pub(crate) enum Payload {
    ClientConnected {
        sender: tokio::sync::mpsc::UnboundedSender<Payload>,
    },
    ClientDisconnected,
    ServerConnected {
        sender: tokio::sync::mpsc::UnboundedSender<Payload>,
    },
    ServerDisconnected,
}

pub(crate) fn backend(app: &mut App, runtime: &tokio::runtime::Runtime) {
    let (control_sender, control_receiver) = tokio::sync::mpsc::unbounded_channel();
    let (inbound_sender, inbound_receiver) = tokio::sync::mpsc::unbounded_channel();

    async fn handle_control(
        mut control_receiver: tokio::sync::mpsc::UnboundedReceiver<Control>,
    ) -> crate::Result<()> {
        while let Some(control) = control_receiver.recv().await {
            match control {
                Control::Connect { address, port } => todo!(),
                Control::Disconnect => todo!(),
            }
        }

        Ok(())
    }

    async fn handle_inbound(
        inbound_sender: tokio::sync::mpsc::UnboundedSender<Data>,
    ) -> crate::Result<()> {
        let mut connection_id = 0;

        loop {
            connection_id += 1;

            let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel();

            if connection_id % 2 == 0 {
                inbound_sender.send(Data {
                    connection_id,
                    payload: Payload::ClientConnected { sender },
                })?;
            } else {
                inbound_sender.send(Data {
                    connection_id,
                    payload: Payload::ServerConnected { sender },
                })?;
            }

            if connection_id % 3 == 0 {
                let connection_id = connection_id / 3;

                if connection_id % 2 == 0 {
                    inbound_sender.send(Data {
                        connection_id: connection_id,
                        payload: Payload::ClientDisconnected,
                    })?;
                } else {
                    inbound_sender.send(Data {
                        connection_id: connection_id,
                        payload: Payload::ServerDisconnected,
                    })?;
                }
            }

            tokio::spawn(async move {
                while let Some(payload) = receiver.recv().await {
                    println!("{:?}", payload);
                }
            });

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }

    app.insert_resource(control_sender);
    app.insert_resource(inbound_receiver);

    runtime.spawn(handle_control(control_receiver));
    runtime.spawn(handle_inbound(inbound_sender));
}

pub(crate) struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Data>()
            .add_system_to_stage(CoreStage::First, inbound);
    }
}

/// Translates inbound messages from tokio runtime to bevy runtime.
fn inbound(
    mut writer: EventWriter<Data>,
    mut inbound_receiver: ResMut<tokio::sync::mpsc::UnboundedReceiver<Data>>,
) {
    while let Ok(inbound) = inbound_receiver.try_recv() {
        writer.send(inbound);
    }
}
