use std::{marker::PhantomData, sync::Arc};

use tokio::{
    net::{ToSocketAddrs, UdpSocket},
    sync::mpsc
};

pub struct Builder<S, T = u64> {
    seed_addrs: Vec<String>,
    dispatcher: T,
    metric: Arc<>
}