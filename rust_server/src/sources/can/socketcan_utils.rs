use futures_util::StreamExt;
use socketcan::{tokio::CanSocket, CanFrame, EmbeddedFrame, Frame};
use std::env;

#[tokio::main]
pub async fn test_can(dbc_fella: dbc_rs::Dbc) -> std::io::Result<()> {
    let iface = env::args().nth(1).unwrap_or_else(|| "vcan0".into());
    let mut sock = CanSocket::open(&iface).unwrap();

    println!("Reading on {}", iface);

    while let Some(res) = sock.next().await {
        match res {
            Ok(CanFrame::Data(frame)) => {
                println!(
                    "{:?}",
                    dbc_fella.decode(frame.can_id().as_raw(), frame.data(), false)
                )
            }
            Ok(CanFrame::Remote(frame)) => println!("{:?}", frame),
            Ok(CanFrame::Error(frame)) => println!("{:?}", frame),
            Err(err) => eprintln!("{}", err),
        }
    }

    Ok(())
}
