use crate::event_stream::EventStream;
use futures_core::stream::Stream;

/// Main entrypoint for creating [`crate::Event`] streams
pub trait Eventsource: Sized {
    /// Create an event stream from a stream of bytes
    fn eventsource(self, add_data: Option<bool>) -> EventStream<Self>;
}

impl<S, B, E> Eventsource for S
where
    S: Stream<Item = Result<B, E>>,
    B: AsRef<[u8]>,
{
    fn eventsource(self, add_data: Option<bool>) -> EventStream<Self> {
        EventStream::new(self, add_data.unwrap_or(false))
    }
}
