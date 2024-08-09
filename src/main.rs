use crate::models::Workspace; // Assuming Workspace is the correct type to use
use serde_json::to_string_pretty;
use std::error::Error;
use std::io;

mod ipc;
mod models;

async fn print_json<T: serde::Serialize>(label: &str, data: T) -> io::Result<()> {
    let json = to_string_pretty(&data)?;
    println!("{} JSON: {}", label, json);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut socket = ipc::WayfireSocket::connect().await?;

    // Get all views and filter those with "role" == "toplevel"
    let views = socket
        .list_views()
        .await?
        .into_iter()
        .filter(|view| view.role == "toplevel")
        .collect::<Vec<_>>();
    let outputs = socket.list_outputs().await?;
    let wsets = socket.list_wsets().await?;
    let input_devices = socket.list_input_devices().await?;

    for view in &views {
        print_json("list_views", view).await?;
    }

    for output in &outputs {
        print_json("list_outputs", output).await?;
    }

    for wset in &wsets {
        print_json("list_wsets", wset).await?;
    }

    print_json("list_input_devices", input_devices).await?;

    match socket.get_configuration().await {
        Ok(config) => print_json("get_configuration", config).await?,
        Err(e) => eprintln!("Failed to get configuration: {}", e),
    }

    match socket.get_option_value("core/plugins").await {
        Ok(response) => print_json("get_option_value", response).await?,
        Err(e) => eprintln!("Failed to get option value: {}", e),
    }

    match socket.get_output(1).await {
        Ok(output) => print_json("get_output", output).await?,
        Err(e) => eprintln!("Failed to get output: {:?}", e),
    }

    if let Some(view) = views.get(0) {
        let view_id = view.id;
        match socket.get_view(view_id).await {
            Ok(detailed_view) => print_json("get_view:", detailed_view).await?,
            Err(e) => eprintln!("Failed to get detailed view: {}", e),
        }

        // Set workspace for the first view
        let target_workspace = Workspace {
            x: 1,
            y: 1,
            grid_width: 4,
            grid_height: 4,
        }; // Adjust as needed
        let output_id = None; // Replace with specific output ID if needed

        match socket
            .set_workspace(target_workspace, view_id, output_id)
            .await
        {
            Ok(_) => println!("Successfully set workspace for view ID: {}", view_id),
            Err(e) => eprintln!("Failed to set workspace: {}", e),
        }
    } else {
        println!("No views found.");
    }

    // toggle expo twice
    match socket.expo_toggle().await {
        Ok(view_alpha) => print_json("toggle expo", view_alpha).await?,
        Err(e) => eprintln!("Failed to toggle expo: {}", e),
    }
    match socket.expo_toggle().await {
        Ok(view_alpha) => print_json("toggle expo", view_alpha).await?,
        Err(e) => eprintln!("Failed to toggle expo: {}", e),
    }

    // toggle scale twice
    match socket.scale_toggle().await {
        Ok(view_alpha) => print_json("toggle scale", view_alpha).await?,
        Err(e) => eprintln!("Failed to toggle expo: {}", e),
    }
    match socket.scale_toggle().await {
        Ok(view_alpha) => print_json("toggle scale", view_alpha).await?,
        Err(e) => eprintln!("Failed to toggle expo: {}", e),
    }

    match socket.cube_activate().await {
        Ok(_) => println!("Cube activated successfully."),
        Err(e) => eprintln!("Failed to activate cube: {}", e),
    }

    match socket.cube_rotate_left().await {
        Ok(_) => println!("Cube rotated left successfully."),
        Err(e) => eprintln!("Failed to rotate cube left: {}", e),
    }

    match socket.cube_rotate_right().await {
        Ok(_) => println!("Cube rotated right successfully."),
        Err(e) => eprintln!("Failed to rotate cube right: {}", e),
    }

    let focused_view = socket.get_focused_view().await?;
    let view_id = focused_view.id;

    match socket.toggle_showdesktop().await {
        Ok(_) => println!("Toggled show desktop successfully."),
        Err(e) => eprintln!("Failed to toggle show desktop: {}", e),
    }

    let state = true;

    let focused_view_id = match socket.get_focused_view().await {
        Ok(view) => view.id,
        Err(e) => {
            eprintln!("Failed to get focused view: {}", e);
            return Err(e);
        }
    };

    // Configure view
    match socket
        .configure_view(focused_view_id, 100, 100, 800, 600, Some(1))
        .await
    {
        Ok(response) => print_json("configure_view", response).await?,
        Err(e) => eprintln!("Failed to configure view: {}", e),
    }

    // Assign slot
    match socket.assign_slot(focused_view_id, "top-left").await {
        Ok(response) => print_json("assign_slot", response).await?,
        Err(e) => eprintln!("Failed to assign slot: {}", e),
    }

    // Set focus
    match socket.set_focus(focused_view_id).await {
        Ok(response) => print_json("set_focus", response).await?,
        Err(e) => eprintln!("Failed to set focus: {}", e),
    }

    match socket.set_view_sticky(view_id, state).await {
        Ok(_) => println!("Set view sticky successfully."),
        Err(e) => eprintln!("Failed to set view sticky: {}", e),
    }

    match socket.send_view_to_back(view_id, state).await {
        Ok(_) => println!("Sent view to back successfully."),
        Err(e) => eprintln!("Failed to send view to back: {}", e),
    }

    match socket.set_view_minimized(view_id, true).await {
        Ok(_) => println!("Set view minimized successfully."),
        Err(e) => eprintln!("Failed to set view minimized: {}", e),
    }

    match socket.set_view_minimized(view_id, false).await {
        Ok(_) => println!("Set view minimized successfully."),
        Err(e) => eprintln!("Failed to set view minimized: {}", e),
    }

    // Get view alpha
    if let Some(view) = views.get(0) {
        let view_id = view.id;
        match socket.get_view_alpha(view_id).await {
            Ok(view_alpha) => print_json("get_view_alpha", view_alpha).await?,
            Err(e) => eprintln!("Failed to get view alpha: {}", e),
        }
    } else {
        println!("No views found.");
    }

    Ok(())
}
