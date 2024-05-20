mod serialization;
mod redis_data_type;
mod server;
mod command;
#[cfg(test)]
mod tests;

fn main() {
    let listener = server::create_server("127.0.0.1", "6379");
    server::listen_for_connections(listener);
}
