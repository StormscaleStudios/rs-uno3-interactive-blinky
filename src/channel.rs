use core::cell::Cell;

pub struct Channel<T> {
    item: Cell<Option<T>>
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Self { item: Cell::new(None) }
    }

    pub fn get_sender(&self) -> Sender<T> {
        Sender { channel: &self }
    }

    pub fn get_reciever(&self) -> Reciever<T> {
        Reciever { channel: &self }
    }

    fn send(&self, item: T) {
        self.item.replace(Some(item));
    }

    fn receive(&self) -> Option<T> {
        self.item.take()
    }
}

pub struct Sender<'a, T> {
    channel: &'a Channel<T>
}

impl<T> Sender<'_, T> {
    pub fn send(&self, item: T) -> () {
        self.channel.send(item);
    }
}

pub struct Reciever<'a, T> {
    channel: &'a Channel<T>
}

impl<T> Reciever<'_, T> {
    pub fn receive(&self) -> Option<T> {
        self.channel.receive()
    }
}
