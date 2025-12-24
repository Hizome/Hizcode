use core::Session;
use tui::app::App;
use tokio::sync::mpsc;
use protocol::AgentEvent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Setup Channels
    let (event_tx, event_rx) = mpsc::channel::<AgentEvent>(100);
    let (input_tx, mut input_rx) = mpsc::channel::<String>(100);

    // 2. Initialize Core Session
    let mut session = Session::new(event_tx.clone());

    // 3. Spawn Core Loop
    tokio::spawn(async move {
        while let Some(input) = input_rx.recv().await {
            session.handle_user_input(input).await;
        }
    });

    // 4. Run TUI (Main Thread)
    let app = App::new(event_rx, input_tx);
    tui::run(app).await?;

    Ok(())
}
