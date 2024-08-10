use std::error::Error;
use wayfire_rs::ipc::WayfireSocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Establish a connection to the Wayfire IPC socket.
    let mut socket = WayfireSocket::connect().await?;
    
    // Extract the ID of the focused view and output.
    let focused_view = socket.get_focused_view().await?;
    let view_id: i64 = focused_view.id;
    let focused_output: i64 = focused_view.output_id;
    
    // move focused view to workspace 4 considering a workspace grid 3x3
    match socket.set_workspace(1, 1, view_id, output_id).await {
        Ok(_) => println!("Successfully set workspace"),
        Err(e) => eprintln!("Failed to set workspace: {}", e),
    }

    Ok(())
}
