# Overview

This is a command line texting interface, you enter an IP and a message, and the program will send it. Messages can be sent using the client or peer. The client however, cannot recieve messages. If you have sent the message to a valid server/peer the message will appear on their end.

Before running the program you will need to install rust, and run
`cargo build`
in the directory you've cloned this to. The binaries will be located in target/debug/(peer/client/server).exe.

This software was developed in order to gain an understanding of networking in rust. I want to try to write a rust backend framework eventually, and as such, need to understand rust networking functions and how to use them.

[Peer Demonstation Video](https://youtu.be/9qPakO4H1mg)
[Client/Server Demonstration Video](https://youtu.be/7djmEJaKkIA)

# Network Communication

We have both a client/server and a peer to peer model included in this program!

We are using TCP because we don't want any messages to be dropped.

The messages are just sent as unencrypted text, terminated by two newline characters.

# Development Environment

Programmed in Rust 1.65.0 (897e37553 2022-11-02)

# Useful Websites

* [Rust Programming: The Book](https://doc.rust-lang.org/book/)
* [Building a single-threaded web server in Rust](https://doc.rust-lang.org/book/ch20-01-single-threaded.html)
* [Changing a single-threaded web server to be multi-threaded in Rust](https://doc.rust-lang.org/book/ch20-02-multithreaded.html)

* NOTE: The reason we could skip part 3 of chapter 20, "Graceful Shutdown and Cleanup", was because we only have the listener operating on a seperate thread, so if a message came in while the program was closing, the final message would simply not be recieved. However, there would be no data malformation, or client handling that was ended abruptly.

# Future Work

* Improved data format, such as sending data using JSON
* Send some form of test data to ensure the server/peer is responding before sending the actual message
* Implement some form of encryption!
* Better error checking/handling
* Be able to see what IP address sends you a message