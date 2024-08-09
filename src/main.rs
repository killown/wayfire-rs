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
    } else {
        println!("No views found.");
    }

    // Get view alpha
    if let Some(view) = views.get(0) {
        let view_id = view.id;
        match socket.get_view_alpha(view_id).await {
            Ok(alpha) => print_json("get_view_alpha:", alpha).await?,
            Err(e) => eprintln!("Failed to get view alpha: {}", e),
        }
    } else {
        println!("No views found.");
    }

    // Get focused view
    match socket.get_focused_view().await {
        Ok(view) => print_json("get_focused_view:", view).await?,
        Err(e) => eprintln!("Failed to get focused view: {}", e),
    }

    // Get focused output
    match socket.get_focused_output().await {
        Ok(output) => print_json("get_focused_output:", output).await?,
        Err(e) => eprintln!("Failed to get focused output: {}", e),
    }

    // Set tiling layout
    let layout = models::Layout {
        geometry: models::Geometry {
            height: 1038,
            width: 2560,
            x: 2560,
            y: 42,
        },
        percent: 1.0,
        vertical_split: vec![],
    };

    let wset_index = 1;
    let workspace_x = 0;
    let workspace_y = 0;

    let response = socket
        .set_tiling_layout(wset_index, workspace_x, workspace_y, &layout)
        .await?;

    println!("Response: {:?}", response);

    // Set view alpha
    let focused_view = socket.get_focused_view().await?;
    let view_id = focused_view.id;
    let alpha = 0.5;
    let response = socket.set_view_alpha(view_id, alpha).await?;
    println!("Response: {:?}", response);
    let alpha = 0.9;
    let response = socket.set_view_alpha(view_id, alpha).await?;
    println!("Response: {:?}", response);

    // Set a view to fullscreen
    if let Some(view) = views.get(0) {
        let view_id = view.id;
        let state = true;
        let state_back: bool = false;
        let response = socket.set_view_fullscreen(view_id, state).await?;
        println!("Set view to fullscreen response: {:?}", response);
        socket.set_view_fullscreen(view_id, state_back).await?;
    }

    // Toggle Expo mode
    let response = socket.toggle_expo().await?;
    println!("Toggle Expo response: {:?}", response);
    let response = socket.toggle_expo().await?;
    println!("Toggle Expo response: {:?}", response);

    Ok(())
}
