use std::error::Error;
use wayfire_rs::ipc::WayfireSocket;
use wayfire_rs::models::WSGeometry;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Establish a connection to the Wayfire IPC socket.
    let mut socket = WayfireSocket::connect().await?;

    // Retrieve the currently focused view and output.
    let focused_view = socket.get_focused_view().await?;
    let focused_output = socket.get_focused_output().await?;

    // Extract the ID of the focused view and output.
    let view_id = focused_view.id;
    let output_id: Option<i64> = Some(focused_output.id);
    let target_workspace: WSGeometry = focused_view.geometry;

    match socket
        .set_workspace(target_workspace, view_id, output_id)
        .await
    {
        Ok(_) => println!("Successfully set workspace"),
        Err(e) => eprintln!("Failed to set workspace: {}", e),
    }

    Ok(())
}
