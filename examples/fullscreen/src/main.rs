use std::error::Error;
use wayfire_rs::ipc::WayfireSocket;

async fn fullscreen_view(socket: &mut WayfireSocket, view_id: i64, state: bool) {
    match socket.set_view_fullscreen(view_id, state).await {
        Ok(_) => println!("Set view fullscreend successfully."),
        Err(e) => eprintln!("Failed to set view fullscreend: {}", e),
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
    let _output_id: Option<i64> = Some(focused_output.id);

    // Set fullscreen the focused view
    fullscreen_view(&mut socket, view_id, true).await;

    Ok(())
}
