use bevy::prelude::*;

pub(crate) fn backend(app: &mut App, runtime: &tokio::runtime::Runtime) {
    let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();

    async fn process(sender: tokio::sync::mpsc::UnboundedSender<u32>) -> crate::Result<()> {
        let mut counter = 0;

        loop {
            sender.send(counter)?;
            counter += 1;
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }

    app.insert_resource(receiver);

    runtime.spawn(async move {
        if let Err(error) = process(sender).await {
            error!(error = ?error, "error");
        }
    });
}

pub(crate) struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Protocol>()
            .add_system_to_stage(CoreStage::PreUpdate, connection_lost);
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Protocol(u32);

fn connection_lost(
    mut receiver: ResMut<tokio::sync::mpsc::UnboundedReceiver<u32>>,
    mut writer: EventWriter<Protocol>,
) {
    while let Ok(counter) = receiver.try_recv() {
        writer.send(Protocol(counter));
    }
}
