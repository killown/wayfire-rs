use serde_json::to_string_pretty;
use std::error::Error;
use std::io;
use wayfire_rs::ipc::WayfireSocket;

async fn print_json<T: serde::Serialize>(label: &str, data: T) -> io::Result<()> {
    let json = to_string_pretty(&data)?;
    println!("{} JSON: {}", label, json);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Establish a connection to the Wayfire IPC socket.
    let mut socket = WayfireSocket::connect().await?;

    // Toggle the expo effect twice 
    match socket.expo_toggle().await {
        Ok(view_alpha) => print_json("toggle expo", view_alpha).await?,
        Err(e) => eprintln!("Failed to toggle expo: {}", e),
    }
    match socket.expo_toggle().await {
        Ok(view_alpha) => print_json("toggle expo", view_alpha).await?,
        Err(e) => eprintln!("Failed to toggle expo: {}", e),
    }

    // Toggle the scale effect twice
    match socket.scale_toggle().await {
        Ok(view_alpha) => print_json("toggle scale", view_alpha).await?,
        Err(e) => eprintln!("Failed to toggle scale: {}", e),
    }
    match socket.scale_toggle().await {
        Ok(view_alpha) => print_json("toggle scale", view_alpha).await?,
        Err(e) => eprintln!("Failed to toggle scale: {}", e),
    }

    // Activate the cube effect
    match socket.cube_activate().await {
        Ok(_) => println!("Cube activated successfully."),
        Err(e) => eprintln!("Failed to activate cube: {}", e),
    }

    // Rotate the cube to the left 
    match socket.cube_rotate_left().await {
        Ok(_) => println!("Cube rotated left successfully."),
        Err(e) => eprintln!("Failed to rotate cube left: {}", e),
    }

    // Rotate the cube to the right 
    match socket.cube_rotate_right().await {
        Ok(_) => println!("Cube rotated right successfully."),
        Err(e) => eprintln!("Failed to rotate cube right: {}", e),
    }

    Ok(())
}
