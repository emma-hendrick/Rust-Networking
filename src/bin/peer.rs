use mrh_rust_lib::services::{
    io_functions::*,
    networking::networking_functions::*,
};

fn main() {

    std::thread::spawn(|| {
        listen(response_callback, "localhost:7878");
    });
    
    loop {
        send_message();
    }

}

fn response_callback(message: String) {
    output(format!("Recieving: {}", message));
}

fn send_message() {

    output("IP Address:");
    let address: String = input();
    output("Message:");
    let message: String = input();
    send(&message, &address);
    
}