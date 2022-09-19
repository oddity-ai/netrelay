use std::env::args;
use tokio::{
  io::{
    AsyncReadExt,
    AsyncWriteExt,
  },
  net::{
    TcpListener,
    TcpStream,
  },
};

#[tokio::main]
async fn main() {
  let args = args()
    .collect::<Vec<String>>();

  match args.len() {
    3 => {
      if let Err(err) = run(&args[1], &args[2]).await {
        println!("error: {}", err);
      }
    },
    _ => help(),
  }
}

fn help() {
  println!("Usage: netrelay <INBOUND> <OUTBOUND>");
}

async fn run(
  addr_inbound: &str,
  addr_outbound: &str,
) -> Result<(), Box<dyn std::error::Error>> {
  let listener = TcpListener::bind(addr_inbound).await?;
  loop {
    let (inbound, _) = listener.accept().await?;
    let (inbound_reader, inbound_writer) = inbound.into_split();

    let outbound = TcpStream::connect(addr_outbound.clone()).await?;
    let (outbound_reader, outbound_writer) = outbound.into_split();

    tokio::spawn(redirect(inbound_reader, outbound_writer));
    tokio::spawn(redirect(outbound_reader, inbound_writer));
  }
}

async fn redirect(
  mut reader: impl AsyncReadExt + Unpin,
  mut writer: impl AsyncWriteExt + Unpin,
) {
  let mut buffer = [0; 1500];
  loop {
    let n = match reader.read(&mut buffer).await {
      Ok(n) if n == 0 => return,
      Ok(n) => n,
      Err(_) => return,
    };

    if writer.write_all(&buffer[0..n]).await.is_err() {
      return;
    }
  }
}