use alloc::{
    boxed::Box,
    collections::BTreeMap,
    sync::Arc,
    task::Wake,
};
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
};

use crossbeam_queue::ArrayQueue;

pub struct Executor {
    tasks: BTreeMap<TaskId, Task>,
    task_queue: Arc<ArrayQueue<TaskId>>,
    waker_cache: BTreeMap<TaskId, Waker>,
}

impl Executor {
    const TASK_QUEUE_CAPACITY: usize = 100;

    pub fn new() -> Self {
        Self {
            tasks: BTreeMap::new(),
            task_queue: Arc::new(ArrayQueue::new(Self::TASK_QUEUE_CAPACITY)),
            waker_cache: BTreeMap::new(),
        }
    }

    pub fn spawn(&mut self, fut: impl Future<Output=()> + 'static) -> &mut Self {
        let task = Task::new(fut);
        let id = task.id;

        self.tasks.insert(id, task);
        self.task_queue.push(id).expect("Executor queue full");

        self
    }

    pub fn run(&mut self) -> ! {
        loop {
            self.run_ready_tasks();

            arch::disable_interrupts();
            match self.task_queue.is_empty() {
                true => arch::enable_interrupts_and_wait(),
                false => arch::enable_interrupts(),
            }
        }
    }

    fn run_ready_tasks(&mut self) {
        let Self {
            tasks,
            task_queue,
            waker_cache,
        } = self;

        while let Some(task_id) = task_queue.pop() {
            let task = match tasks.get_mut(&task_id) {
                Some(task) => task,
                None => continue,
            };

            let waker = waker_cache
                .entry(task_id)
                .or_insert_with(|| TaskWaker::new(task_id, task_queue.clone()));
            let mut cx = Context::from_waker(waker);

            match task.poll(&mut cx) {
                Poll::Pending => {},
                Poll::Ready(_) => {
                    tasks.remove(&task_id);
                    waker_cache.remove(&task_id);
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct TaskId(u64);

impl TaskId {
    fn new() -> Self {
        use core::sync::atomic::{AtomicU64, Ordering};
        static ID: AtomicU64 = AtomicU64::new(0);

        Self(ID.fetch_add(1, Ordering::Relaxed))
    }
}

struct Task {
    id: TaskId,
    fut: Pin<Box<dyn Future<Output=()> + 'static>>,
}

impl Task {
    fn new(fut: impl Future<Output=()> + 'static) -> Self {
        Self {
            id: TaskId::new(),
            fut: Box::pin(fut),
        }
    }

    fn poll(&mut self, cx: &mut Context) -> Poll<()> {
        self.fut.as_mut().poll(cx)
    }
}

#[derive(Clone)]
struct TaskWaker {
    task_id: TaskId,
    task_queue: Arc<ArrayQueue<TaskId>>,
}

impl TaskWaker {
    fn new(task_id: TaskId, task_queue: Arc<ArrayQueue<TaskId>>) -> Waker {
        Waker::from(Arc::new(Self {
            task_id,
            task_queue,
        }))
    }
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.task_queue
            .push(self.task_id)
            .expect("Executor queue full");
    }
}
