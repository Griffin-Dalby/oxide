extern crate tokio;

use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

// Define ServerController
pub struct ServerController {
    
}

impl ServerController {
    pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind("127.0.0.1:0814").await?;
        
        loop {
            let (socket, _) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                process(socket).await;
            });
        }
    }
}

// Server Behavior
async fn process(socket: TcpStream) {
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}