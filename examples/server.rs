use lib60870::server::ServerBuilder;
use lib60870::types::{CauseOfTransmission, Quality};

fn main() {
    let mut server = ServerBuilder::new()
        .local_port(2404)
        .build()
        .expect("Failed to create server");

    server.set_connection_event_handler(|event| {
        println!("Connection: {:?}", event);
    });

    server.set_interrogation_handler(|conn, asdu, qoi| {
        println!("Interrogation for group {}", qoi);
        conn.send_act_con(&asdu, false);
        // Send response data here...
        conn.send_act_term(&asdu);
        true
    });

    server.start();
    println!("Server running on port 2404");

    // Send periodic data
    loop {
        server.send_measured_scaled(
            CauseOfTransmission::Periodic,
            1,    // Common address
            100,  // IOA
            42,   // Value
            Quality::GOOD,
        );
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

