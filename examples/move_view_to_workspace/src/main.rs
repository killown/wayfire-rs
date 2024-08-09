use serde_json::json;
use std::error::Error;
use wayfire_rs::ipc::WayfireSocket;
use wayfire_rs::models::{MsgTemplate, Workspace};

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

    // Define the target workspace where the view will be moved.
    let target_workspace = Workspace {
        x: 1,           // Column index of the target workspace.
        y: 1,           // Row index of the target workspace.
        grid_width: 4,  // Total number of columns in the workspace grid.
        grid_height: 4, // Total number of rows in the workspace grid.
    };

    // Create the message template for setting the workspace.
    let message = MsgTemplate {
        method: "vswitch/set-workspace".to_string(), // IPC method to set the workspace.
        data: Some(json!({
            "x": target_workspace.x, // X-coordinate of the target workspace.
            "y": target_workspace.y, // Y-coordinate of the target workspace.
            "output-id": output_id, // ID of the output where the view should be moved.
            "view-id": view_id // ID of the view to be moved.
        })),
    };

    // Send the message to the Wayfire IPC socket.
    socket.send_json(&message).await?;

    Ok(())
}
