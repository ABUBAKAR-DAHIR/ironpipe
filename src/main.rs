mod server;
mod config;
fn main() {
    println!("Starting the server...");
    server::tcp::start();
}
