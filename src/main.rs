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
