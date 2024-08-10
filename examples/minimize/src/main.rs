use std::error::Error;
use tokio::time::{sleep, Duration};
use wayfire_rs::ipc::WayfireSocket;

async fn minimize_view(socket: &mut WayfireSocket, view_id: i64, state: bool) {
    match socket.set_view_minimized(view_id, state).await {
        Ok(_) => println!("Set view minimized successfully."),
        Err(e) => eprintln!("Failed to set view minimized: {}", e),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Establish a mutable connection to the Wayfire IPC socket.
    let mut socket = WayfireSocket::connect().await?;

    // Retrieve the currently focused view and output.
    let focused_view = socket.get_focused_view().await?;
    let focused_output = socket.get_focused_output().await?;

    // Extract the ID of the focused view and output.
    let view_id = focused_view.id;
    let _output_id: Option<i64> = Some(focused_output.id); // Suppressed warning

    // Minimize the focused view and unminimize
    minimize_view(&mut socket, view_id, true).await;
    sleep(Duration::from_secs(1)).await;
    minimize_view(&mut socket, view_id, false).await;

    Ok(())
}
