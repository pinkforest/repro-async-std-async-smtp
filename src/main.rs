use async_smtp::{SmtpClient, SmtpTransport};
use async_std::net::TcpStream;

#[async_std::main]
async fn main() {

    let stream = TcpStream::connect("localhost:8025").await.unwrap();
    let mut client = SmtpClient::new();
    let mut transport = SmtpTransport::new(client, stream).await.unwrap();

}
