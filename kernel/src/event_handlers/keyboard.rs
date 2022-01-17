use core::pin::Pin;
use core::task::{Context, Poll};

use crossbeam_queue::ArrayQueue;
use futures_util::{Stream, StreamExt};
use futures_util::task::AtomicWaker;

use libcore::lazy::Once;

static SCANCODES: Once<ArrayQueue<u8>> = Once::new();
static WAKER: AtomicWaker = AtomicWaker::new();

#[no_mangle]
pub extern "C" fn add_keyboard_scan_code(scancode: u8) {
    let _ = SCANCODES
        .get()
        .expect("Scancode queue uninitialized")
        .push(scancode)
        .map(|_| WAKER.wake())
        .map_err(|_| log::warn!("scancode queue full, dropping keyboard input"));
}

pub async fn print_key_presses() {
    use pc_keyboard::{Keyboard, ScancodeSet1, layouts::Uk105Key, HandleControl, DecodedKey};

    let mut stream = ScancodeStream::new();
    let mut keyboard = Keyboard::new(
        Uk105Key,
        ScancodeSet1,
        HandleControl::Ignore,
    );

    while let Some(scancode) = stream.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(c) => log::info!("{}", c),
                    DecodedKey::RawKey(k) => log::info!("{:?}", k),
                }
            }
        }
    }
}

struct ScancodeStream {
    _private: (),
}

impl ScancodeStream {
    const QUEUE_CAPACITY: usize = 100;

    fn new() -> Self {
        SCANCODES
            .try_set(|| ArrayQueue::new(Self::QUEUE_CAPACITY))
            .expect("ScancodeStream::new should only be called once");
        Self { _private: () }
    }
}

impl Stream for ScancodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let queue = SCANCODES
            .get()
            .unwrap();

        if let Some(code) = queue.pop() {
            return Poll::Ready(Some(code));
        }

        WAKER.register(cx.waker());
        match queue.pop() {
            Some(code) => {
                WAKER.take();
                Poll::Ready(Some(code))
            }
            None => Poll::Pending,
        }
    }
}
