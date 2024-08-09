use std::error::Error;
use wayfire_rs::ipc::WayfireSocket;

async fn print_info() -> Result<(), Box<dyn Error>> {
    let mut socket = WayfireSocket::connect().await?;

    // Example calls to WayfireSocket methods
    let views = socket.list_views().await?;
    let outputs = socket.list_outputs().await?;
    let wsets = socket.list_wsets().await?;
    let input_devices = socket.list_input_devices().await?;

    println!("Views: {:?}", views);
    println!("Outputs: {:?}", outputs);
    println!("Workspaces: {:?}", wsets);
    println!("Input Devices: {:?}", input_devices);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    print_info().await?;
    Ok(())
}
