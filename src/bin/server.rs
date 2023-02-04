use mrh_rust_lib::services::{
    io_functions::*,
    networking::networking_functions::*,
};

fn main() {
    listen(response_callback, "localhost:7878");
}

fn response_callback(message: String) {
    output(format!("Recieving: {}", message));
}
