mod server;
mod router;
mod handler;
use server::Server;
fn main() {
    let s=Server::new("localhost:3000");
    s.run();
}
