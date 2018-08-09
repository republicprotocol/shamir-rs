extern crate futures;

use std::sync::mpsc;
use communicator::futures::future::Future;
use communicator::futures::prelude::Async;
use communicator::futures::task::Context;

enum Error {}

trait Response<T> 
    where T: Future {
    fn send(&self, T::Item) -> T;
}

struct Communicator<'a, T: 'a, U: 'a>
    where U: PartialEq {
    id: U,
    msg: T,
    receiver: &'a mpsc::Receiver<(U, T)>,
    sender: &'a mpsc::Sender<(U, T)>,
}

impl<'a, T, U> Future for Communicator<'a, T, U> {
    type Item = T;
    type Error = Error;

    fn poll(&mut self, cx: &mut Context) -> Result<Async<Self::Item>, Self::Error> {
        unimplemented!()
    }
}

impl<'a, T> Response<T> for Communicator<'a, T::Item>
    where T: Future {
    fn send(&self, msg: T::Item) -> T {
        self.sender.send(msg);
        unimplemented!()
    }
}