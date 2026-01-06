# Dispatch

Dispatch is a lightweight, native HTTP client for Linux, built with Rust and GTK. It is inspired by tools like Postman and Insomnia, but aims to be a more resource-efficient and integrated solution for the Linux desktop.

## Features

*   **Lightweight and Fast:** Built with Rust, Dispatch is designed to be fast and consume minimal system resources.
*   **Native Linux Experience:** Uses GTK4 and Libadwaita to provide a modern, native look and feel on the Linux desktop.
*   **HTTP Requests:** Send and manage HTTP requests with a simple and intuitive interface.
*   **JSON Syntax Highlighting:** The response view uses Sourceview5 for JSON syntax highlighting.
*   **Local History:** Requests are stored locally in a SQLite database.

## Getting Started

### Prerequisites

*   Rust
*   GTK4
*   Libadwaita
*   Sourceview5

### Building from Source

1.  Clone the repository:
    ```sh
    git clone https://github.com/your-username/dispatch.git
    ```
2.  Build the project:
    ```sh
    cd dispatch
    cargo build
    ```
3.  Run the application:
    ```sh
    cargo run
    ```

## Contributing

Contributions are welcome! If you have a feature request, bug report, or want to contribute to the code, please open an issue or pull request.

## Project Structure

*   `src/main.rs`: The main entry point of the application.
*   `src/ui/`: Contains the UI components, built with GTK.
*   `src/api/`: Handles the logic for making HTTP requests.
*   `src/database.rs`: Manages the local SQLite database.
*   `src/config.rs`: Handles application configuration.

## Dependencies

*   [gtk4](https://crates.io/crates/gtk4)
*   [libadwaita](https://crates.io/crates/libadwaita)
*   [sourceview5](https://crates.io/crates/sourceview5)
*   [reqwest](https://crates.io/crates/reqwest)
*   [serde_json](https://crates.io/crates/serde_json)
*   [rusqlite](https://crates.io/crates/rusqlite)
*   [chrono](https://crates.io/crates/chrono)
*   [directories](https://crates.io/crates/directories)
