Simple TCP Server in Rust

This project implements a very simple TCP server using Rust’s standard library (std::net::TcpListener and std::net::TcpStream).
It listens for incoming connections, reads client messages, and responds accordingly.

You can run the project locally using Cargo.

🚀 Getting Started
Prerequisites

Rust (stable recommended)

Cargo (comes bundled with Rust)

Check your installation:

rustc --version
cargo --version

Running the Project

Build the project:

cargo build


Run the server:

cargo run


By default, the server will start listening on the configured address 127.0.0.1:8181.

🧵 Next Steps

The current implementation handles requests sequentially — only one client at a time.
The next goal is to make the server multi-threaded, allowing simultaneous connections. A few possible approaches:

Spawn a new thread for each connection

Use a thread pool

Explore crates like tokio (async) or keep it purely in the standard library

📁 Project Structure
src/
 └── main.rs   # TCP server implementation

💡 Contribution Ideas

Add graceful shutdown
Improve logging
Add client examples

Add multi-threaded handling (coming next)
