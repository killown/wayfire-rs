# Wayfire-rs

`wayfire-rs` is a Rust library for interacting with Wayfire, a Wayland compositor. This project provides functionalities to communicate with Wayfire using IPC, manage views, outputs, and configurations, and more.

## Features

- **Connect to Wayfire**: Establish a connection to the Wayfire socket.
- **Send and Receive JSON Messages**: Send JSON messages to Wayfire and handle responses.
- **List and Manage Views**: Retrieve information about and manage views.
- **List and Manage Outputs**: Get details about outputs connected to Wayfire.
- **Retrieve Configuration**: Access Wayfire's configuration details.
- **Handle Input Devices**: Get information about input devices.
- **Manage Workspaces**: Retrieve workspace details and manage workspace sets.

## Getting Started

### Prerequisites

- **Rust**: Ensure you have Rust installed. You can install Rust using [rustup](https://rustup.rs/).

### Installation

To include `wayfire-rs` in your Rust project, add it to your `Cargo.toml`:

```toml
[dependencies]
wayfire-rs = "0.1.8"
```

### Usage

Basic usage in wayfire-rs/examples folder and lots of examples in wayfire-rs/src/main.rs

### API Methods

    list_views: Retrieves a list of views.
    list_outputs: Retrieves a list of outputs.
    list_wsets: Retrieves a list of workspace sets.
    list_input_devices: Retrieves a list of input devices.
    get_configuration: Retrieves Wayfire's configuration.
    get_option_value: Retrieves the value of a specific configuration option.
    get_output: Retrieves information about a specific output.
    get_view: Retrieves information about a specific view.
    get_focused_view: Retrieves information about the currently focused view.
    get_focused_output: Retrieves information about the currently focused output.

### Contributing

If you want to contribute to the wayfire-rs project, follow these steps:

    Fork the repository.
    Create a new branch for your feature or bug fix.
    Make your changes and test them.
    Submit a pull request with a detailed description of your changes.

### License

```
wayfire-rs is licensed under the MIT License.

```
