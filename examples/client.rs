use lib60870::client::ConnectionBuilder;
use lib60870::types::{CauseOfTransmission, QOI_STATION};

fn main() {
    let mut conn = ConnectionBuilder::new("127.0.0.1", 2404)
        .originator_address(3)
        .build()
        .expect("Failed to create connection");

    // Set up handlers
    conn.set_handlers(
        |event| println!("Connection event: {:?}", event),
        |asdu| {
            println!("Received ASDU: {:?}", asdu);
            for obj in asdu.parse_objects() {
                println!("  {:?}", obj);
            }
            true
        },
    );

    // Connect and send interrogation
    if conn.connect() {
        println!("Connected!");
        conn.send_start_dt();
        conn.send_interrogation(CauseOfTransmission::Activation, 1, QOI_STATION);

        std::thread::sleep(std::time::Duration::from_secs(5));
    }
    // Connection automatically closed on drop
}

