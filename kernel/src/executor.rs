use alloc::boxed::Box;
use alloc::collections::VecDeque;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, RawWaker, Waker};

type Task = Pin<Box<dyn Future<Output=()> + 'static>>;

pub struct Executor {
    tasks: VecDeque<Task>,
}

impl Executor {
    pub fn new() -> Self {
        Self { tasks: VecDeque::new() }
    }

    pub fn spawn(&mut self, task: impl Future<Output=()> + 'static) {
        self.tasks.push_back(Box::pin(task))
    }

    pub fn run(&mut self) {
        while let Some(mut task) = self.tasks.pop_front() {
            let waker = dummy_waker();
            let mut context = Context::from_waker(&waker);

            match task.as_mut().poll(&mut context) {
                Poll::Pending => self.tasks.push_back(task),
                Poll::Ready(_) => { /* task is finished */ }
            }
        }
    }
}


fn dummy_raw_waker() -> RawWaker {
    use core::task::RawWakerVTable;

    fn no_op(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        dummy_raw_waker()
    }

    let vtable = &RawWakerVTable::new(clone, no_op, no_op, no_op);
    RawWaker::new(0 as *const (), vtable)
}

fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}
