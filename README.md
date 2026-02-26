# ironpipe

A lightweight TCP server written in Rust.

`ironpipe` is a minimal foundation for learning and building network services. It listens for incoming TCP connections, reads client data, logs what it receives, and sends a simple response back.

## Highlights

- Small, readable Rust codebase
- Config-driven server address and buffer size
- Handles incoming clients over TCP
- Logs connection and payload details
- Returns a server response to each connected client

## Current Behavior

The server currently:

1. Binds to an IP address for example `127.0.0.1:7878`
2. Accepts incoming TCP connections in a loop
3. Reads up to `1024` bytes from each client
4. Prints the received payload
5. Sends back a response, example `Hello from the TCP server`

## Configuration

Server settings are defined in `src/config.rs`:

- `SERVER_ADDRESS`: `127.0.0.1:7878`
- `BUFFER_SIZE`: `1024`

## Project Layout

```text
ironpipe/
|- src/
|  |- config.rs
|  |- main.rs
|  |- server/
|     |- mod.rs
|     |- tcp.rs
|- Cargo.toml
|- README.md
```

## Getting Started

### Prerequisites

- Rust (stable)
- Cargo

### Run the server

```bash
cargo run
```

You should see output similar to:

```text
Starting the server...
Server is listening on: 127.0.0.1:7878
```

## Try It Locally

Connect with `nc` from another terminal:

```bash
echo "hello ironpipe" | nc 127.0.0.1 7878 | telnet 127.0.0.1 7878
```

Or on Windows PowerShell:

```powershell
$client = [System.Net.Sockets.TcpClient]::new("127.0.0.1", 7878)
$stream = $client.GetStream()
$data = [System.Text.Encoding]::UTF8.GetBytes("hello ironpipe")
$stream.Write($data, 0, $data.Length)
$buffer = New-Object byte[] 1024
$read = $stream.Read($buffer, 0, $buffer.Length)
[System.Text.Encoding]::UTF8.GetString($buffer, 0, $read)
$client.Close()
```

Expected response:

```text
Hello from the TCP server
```

## Development

### Build

```bash
cargo build
```

### Run checks

```bash
cargo check
```

### Format

```bash
cargo fmt
```

## Roadmap Ideas

- Handle clients concurrently
- Add graceful shutdown support
- Parse command/message protocol
- Add tests for server behavior
- Add configurable responses
- Add logging levels


## 👤 Author

Built by **Abubakar Dahir Hassan**.

## 📄 License

This project is distributed under the MIT license