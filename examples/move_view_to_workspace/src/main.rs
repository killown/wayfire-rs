use wayfire_rs::ipc::WayfireSocket;

#[tokio::main]
async fn main() {
    // Establish a connection to the Wayfire IPC socket.
    let mut socket = match WayfireSocket::connect().await {
        Ok(sock) => sock,
        Err(e) => {
            eprintln!("Failed to connect to Wayfire IPC socket: {}", e);
            return;
        }
    };

    // Extract the ID of the focused view and output.
    let focused_view = match socket.get_focused_view().await {
        Ok(view) => view,
        Err(e) => {
            eprintln!("Failed to get focused view: {}", e);
            return;
        }
    };
    let view_id: i64 = focused_view.id;
    let output_id: i64 = focused_view.output_id;

    // Move focused view to workspace 4 considering a workspace grid 3x3
    match socket.set_workspace(1, 1, view_id, output_id).await {
        Ok(_) => println!("Successfully set workspace"),
        Err(e) => eprintln!("Failed to set workspace: {}", e),
    }
}
