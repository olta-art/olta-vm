use tokio::sync::mpsc;

pub(crate) type Subscriber = mpsc::UnboundedSender<String>;
