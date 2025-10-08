use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;

pub(crate) type Subscriber = WebSocketStream<TcpStream>;