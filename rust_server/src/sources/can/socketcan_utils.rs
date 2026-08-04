use futures_util::StreamExt;
use socketcan::{tokio::CanSocket, CanFrame, EmbeddedFrame, Frame};
use std::env;

use crate::sources::can::CanLogger;

#[tokio::main]
pub async fn test_joe(can_logger: CanLogger) -> std::io::Result<()> {
    let iface = env::args().nth(1).unwrap_or_else(|| "vcan0".into());
    let mut sock = CanSocket::open(&iface).unwrap();

    println!("Reading on {}", iface);

    while let Some(res) = sock.next().await {
        match res {
            Ok(CanFrame::Data(frame)) => {
                // println!(
                //     "Data: {:?}",
                //     can_logger
                //         .dbc
                //         .decode(frame.can_id().as_raw(), frame.data(), false)
                //         .unwrap()
                // );

                // can_logger.dbc.decode(frame.can_id, payload, is_extended)
                println!(
                    "Message Name: {:?}",
                    can_logger
                        .dbc
                        .messages()
                        .find_by_id(frame.can_id().as_raw())
                        .unwrap() // .name()
                                  // .to_ascii_lowercase()
                );

                // can_logger
            }
            Ok(CanFrame::Remote(frame)) => println!("Remote frame: {:?}", frame),
            Ok(CanFrame::Error(frame)) => println!("Error frame: {:?}", frame),
            Err(err) => eprintln!("{}", err),
        }
    }

    Ok(())
}
