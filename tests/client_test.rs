use embedded_recruitment_task::{
    message::{client_message, server_message, AddRequest, EchoMessage},
    server::Server,
};
use std::{
    sync::Arc,
    thread::{self, JoinHandle},
    time::Duration,
};
use rand::{thread_rng, Rng};
use log::trace;

mod client;

fn setup_server_thread(server: Arc<Server>) -> JoinHandle<()> {
    thread::spawn(move || {
        server.run().expect("Server encountered an error");
    })
}

fn create_server() -> Arc<Server> {
    Arc::new(Server::new("localhost:0").expect("Failed to start server"))
}

// Helper function to create a random delay
fn random_delay(max_ms: u64) {
    let mut rng = thread_rng();
    thread::sleep(Duration::from_millis(rng.gen_range(0..max_ms)));
}

#[test]
fn test_client_connection() {

    // activate logging
    let _ = env_logger::try_init();

    trace!("1 : test_client_connection Start.");

    // Set up the server in a separate thread
    let server = create_server();
    let handle = setup_server_thread(server.clone());
    let server_port = server.get_port();

    // Create and connect the client
    let mut client = client::Client::new("localhost", server_port.into(), 1000);
    assert!(client.connect().is_ok(), "Failed to connect to the server");

    // Disconnect the client
    assert!(
        client.disconnect().is_ok(),
        "Failed to disconnect from the server"
    );

    trace!("Client Disconnected from the server.");

    // Stop the server and wait for thread to finish
    trace!("Stop the server.");
    server.stop();
    trace!("Waiting for Server to End.");
    assert!(
        handle.join().is_ok(),
        "Server thread panicked or failed to join"
    );
}

#[test]
fn test_client_echo_message() {

    // activate logging
    let _ = env_logger::try_init();
    trace!("2 : test_client_echo_message Start.");

    // Set up the server in a separate thread
    let server = create_server();
    let handle = setup_server_thread(server.clone());
    let server_port = server.get_port();

    // Create and connect the client
    let mut client = client::Client::new("localhost", server_port.into(), 1000);

    assert!(client.connect().is_ok(), "Failed to connect to the server");

    // Prepare the message
    let mut echo_message = EchoMessage::default();
    echo_message.content = "Hello, World!".to_string();
    let message = client_message::Message::EchoMessage(echo_message.clone());

    // Send the message to the server
    assert!(client.send(message).is_ok(), "Failed to send message");

    // Receive the echoed message
    let response = client.receive();
    assert!(
        response.is_ok(),
        "Failed to receive response for EchoMessage"
    );

    match response.unwrap().message {
        Some(server_message::Message::EchoMessage(echo)) => {
            assert_eq!(
                echo.content, echo_message.content,
                "Echoed message content does not match"
            );
        }
        _ => panic!("Expected EchoMessage, but received a different message"),
    }

    // Disconnect the client
    assert!(
        client.disconnect().is_ok(),
        "Failed to disconnect from the server"
    );

    // Stop the server and wait for thread to finish
    server.stop();
    assert!(
        handle.join().is_ok(),
        "Server thread panicked or failed to join"
    );
}

#[test]
fn test_multiple_echo_messages() {
    // activate logging
    let _ = env_logger::try_init();
    trace!("3 : test_multiple_echo_messages Start.");

    // Set up the server in a separate thread
    let server = create_server();
    let handle = setup_server_thread(server.clone());

    let server_port = server.get_port();

    // Create and connect the client
    let mut client = client::Client::new("localhost", server_port.into(), 1000);

    assert!(client.connect().is_ok(), "Failed to connect to the server");

    // Prepare multiple messages
    let messages = vec![
        "Hello, World!".to_string(),
        "How are you?".to_string(),
        "Goodbye!".to_string(),
    ];

    trace!("Messages will be sent : {:?}", messages);
    println!("Messages will be sent : {:?}", messages);

    // Send and receive multiple messages
    for message_content in messages {
        let mut echo_message = EchoMessage::default();
        echo_message.content = message_content.clone();
        let message = client_message::Message::EchoMessage(echo_message);

        trace!("Send Message : {:?}", message);

        // Send the message to the server
        assert!(client.send(message).is_ok(), "Failed to send message");

        // Receive the echoed message
        let response = client.receive();
        assert!(
            response.is_ok(),
            "Failed to receive response for EchoMessage"
        );

        match response.unwrap().message {
            Some(server_message::Message::EchoMessage(echo)) => {
                assert_eq!(
                    echo.content, message_content,
                    "Echoed message content does not match"
                );
            }
            _ => panic!("Expected EchoMessage, but received a different message"),
        }
    }

    // Disconnect the client
    assert!(
        client.disconnect().is_ok(),
        "Failed to disconnect from the server"
    );

    // Stop the server and wait for thread to finish
    server.stop();
    assert!(
        handle.join().is_ok(),
        "Server thread panicked or failed to join"
    );
}

#[test]
fn test_multiple_clients() {
    // Set up the server in a separate thread
    let server = create_server();
    let handle = setup_server_thread(server.clone());
    let server_port = server.get_port();

    // Create and connect multiple clients
    let mut clients = vec![
        client::Client::new("localhost", server_port.into(), 1000),
        client::Client::new("localhost", server_port.into(), 1000),
        client::Client::new("localhost", server_port.into(), 1000),
    ];

    for client in clients.iter_mut() {
        assert!(client.connect().is_ok(), "Failed to connect to the server");
    }

    // Prepare multiple messages
    let messages = vec![
        "Hello, World!".to_string(),
        "How are you?".to_string(),
        "Goodbye!".to_string(),
    ];

    // Send and receive multiple messages for each client
    for message_content in messages {
        let mut echo_message = EchoMessage::default();
        echo_message.content = message_content.clone();
        let message = client_message::Message::EchoMessage(echo_message.clone());

        for client in clients.iter_mut() {
            // Send the message to the server
            assert!(
                client.send(message.clone()).is_ok(),
                "Failed to send message"
            );

            // Receive the echoed message
            let response = client.receive();
            assert!(
                response.is_ok(),
                "Failed to receive response for EchoMessage"
            );

            match response.unwrap().message {
                Some(server_message::Message::EchoMessage(echo)) => {
                    assert_eq!(
                        echo.content, message_content,
                        "Echoed message content does not match"
                    );
                }
                _ => panic!("Expected EchoMessage, but received a different message"),
            }
        }
    }

    // Disconnect the clients
    for client in clients.iter_mut() {
        assert!(
            client.disconnect().is_ok(),
            "Failed to disconnect from the server"
        );
    }

    // Stop the server and wait for thread to finish
    server.stop();
    assert!(
        handle.join().is_ok(),
        "Server thread panicked or failed to join"
    );
}

#[test]
fn test_client_add_request() {
    // activate logging
    let _ = env_logger::try_init();
    trace!("5 : test_client_add_request Start.");

    // Set up the server in a separate thread
    let server = create_server();
    let handle = setup_server_thread(server.clone());

    let server_port = server.get_port();

    // Create and connect the client
    let mut client = client::Client::new("localhost", server_port.into(), 1000);
    assert!(client.connect().is_ok(), "Failed to connect to the server");

    trace!("Client connected to the server.");

    // Prepare the message
    let mut add_request = AddRequest::default();
    add_request.a = 10;
    add_request.b = 20;
    let message = client_message::Message::AddRequest(add_request.clone());

    // Send the message to the server
    assert!(client.send(message).is_ok(), "Failed to send message");

    trace!("Client send AddRequest message : {:?} to the server.",add_request);

    // Receive the response
    let response = client.receive();
    trace!("Server responded with: {:?}", response);
    assert!(
        response.is_ok(),
        "Failed to receive response for AddRequest"
    );

    match response.unwrap().message {
        Some(server_message::Message::AddResponse(add_response)) => {
            assert_eq!(
                add_response.result,
                add_request.a + add_request.b,
                "AddResponse result does not match"
            );
        }
        _ => panic!("Expected AddResponse, but received a different message"),
    }

    // Disconnect the client
    assert!(
        client.disconnect().is_ok(),
        "Failed to disconnect from the server"
    );

    // Stop the server and wait for thread to finish
    server.stop();
    assert!(
        handle.join().is_ok(),
        "Server thread panicked or failed to join"
    );
}

#[test]
fn test_multiple_clients_in_threads() {
    // Initialize logging (if not already done)
    let _ = env_logger::try_init();
    log::trace!("6 : test_concurrent_clients_in_threads Start.");

    // 1. Spin up the server in its own thread
    let server = create_server();
    let handle = setup_server_thread(server.clone());
    let server_port = server.get_port();

    // 2. Configure concurrency: number of client threads, messages per client
    let num_clients = 4;
    let messages_per_client = 3;

    // 3. Spawn multiple client threads
    let mut client_threads = Vec::new();
    for client_id in 0..num_clients {
        
        client_threads.push(std::thread::spawn(move || {
            log::trace!("Client {client_id} thread started.");
            // a) Create & connect a client
            let mut client = client::Client::new("localhost", server_port.into(), 3000);
            log::trace!("Client {client_id}: connecting to the server...");

            assert!(
                client.connect().is_ok(),
                "Client {client_id} failed to connect in concurrency test"
            );

            // b) Send multiple messages, receive responses
            for msg_index in 0..messages_per_client {
                // Build an EchoMessage
                let echo_content = format!("Client {client_id} - Message {msg_index}");
                let mut echo_msg = EchoMessage::default();
                echo_msg.content = echo_content.clone();

                let send_msg = client_message::Message::EchoMessage(echo_msg);

                // Log the outgoing message
                log::trace!(
                    "Client {client_id}: sending message {msg_index} => \"{}\"",
                    echo_content
                );

                assert!(
                    client.send(send_msg).is_ok(),
                    "Client {client_id} failed to send message {msg_index}"
                );

                // Receive response
                let response = client.receive();
                log::trace!("Client {client_id}: received response => {:?}", response);

                assert!(
                    response.is_ok(),
                    "Client {client_id} failed to receive echo for message {msg_index}"
                );

                match response.unwrap().message {
                    Some(server_message::Message::EchoMessage(echo_resp)) => {
                        // Confirm echo matches
                        assert_eq!(
                            echo_resp.content, echo_content,
                            "Echo mismatch for client {client_id}, message {msg_index}"
                        );
                        log::trace!(
                            "Client {client_id}, message {msg_index}: echo matched!"
                        );
                    }
                    other => panic!(
                        "Client {client_id} got unexpected response: {:?}",
                        other
                    ),
                }
            }

            // c) Disconnect after sending all messages
            log::trace!("Client {client_id}: disconnecting.");
            assert!(
                client.disconnect().is_ok(),
                "Client {client_id} failed to disconnect"
            );

            log::trace!("Client {client_id} thread done.");
        }));
    }

    // 4. Join all client threads
    for (i, thread) in client_threads.into_iter().enumerate() {
        thread
            .join()
            .expect(&format!("Client thread {} panicked", i));
    }

    // 5. Stop the server and join the server thread
    server.stop();
    assert!(
        handle.join().is_ok(),
        "Server thread panicked or failed to join in concurrency test"
    );

    log::trace!("test_concurrent_clients_in_threads complete.");
}
