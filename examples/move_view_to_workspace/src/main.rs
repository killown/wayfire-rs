use std::error::Error;
use wayfire_rs::ipc::WayfireSocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Establish a connection to the Wayfire IPC socket.
    let mut socket = WayfireSocket::connect().await?;

    // Retrieve the currently focused view and output.
    let focused_view = socket.get_focused_view().await?;
    let focused_output = focused_view.output_id;

    // Extract the ID of the focused view and output.
    let view_id: i64 = focused_view.id;
    let output_id: Option<i64> = Some(focused_output);
    let x: i64 = focused_view.geometry.x;
    let y: i64 = focused_view.geometry.y;

    match socket.set_workspace(x, y, view_id, output_id).await {
        Ok(_) => println!("Successfully set workspace"),
        Err(e) => eprintln!("Failed to set workspace: {}", e),
    }

    Ok(())
}
