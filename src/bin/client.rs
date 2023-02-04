use mrh_rust_lib::services::{
    io_functions::*,
    networking::networking_functions::*,
};

fn main() {
    
    loop {
        send_message();
    }
    
}

fn send_message() {

    output("IP Address:");
    let address: String = input();
    output("Message:");
    let message: String = input();
    send(&message, &address);
    
}