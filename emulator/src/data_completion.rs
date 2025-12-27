use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CompletionStatus {
    Success,
    Failure(String),
    Timeout,
}

struct CompletionCondition {
    status: Mutex<Option<CompletionStatus>>,
    condition: Condvar,
}

impl CompletionCondition {
    pub fn new() -> Self {
        Self {
            status: Mutex::new(None),
            condition: Condvar::new(),
        }
    }
}

pub struct DataWithCompletion<T> {
    data: T,
    processed: bool,
    sync: Arc<CompletionCondition>,
}

impl<T> DataWithCompletion<T> {
    pub fn process(&mut self, function: impl FnOnce(&mut T) -> CompletionStatus) {
        if self.processed {
            panic!("Data has already been processed");
        }
        self.processed = true;
        let result = function(&mut self.data);
        self.complete(result);
    }

    fn complete(&mut self, value: CompletionStatus) {
        let mut status = self.sync.status.lock().unwrap();
        *status = Some(value);
        self.sync.condition.notify_all();
    }
}

pub struct CompletionReceiver {
    sync: Arc<CompletionCondition>,
}

impl CompletionReceiver {
    pub fn wait(&self) -> CompletionStatus {
        let value = self
            .sync
            .condition
            .wait_while(self.sync.status.lock().unwrap(), Self::is_still_processing)
            .unwrap();
        (*value).clone().unwrap()
    }

    pub fn wait_timeout(&self, timeout: Duration) -> CompletionStatus {
        let (value, timeout_result) = self
            .sync
            .condition
            .wait_timeout_while(
                self.sync.status.lock().unwrap(),
                timeout,
                Self::is_still_processing,
            )
            .unwrap();
        if timeout_result.timed_out() {
            CompletionStatus::Timeout
        } else {
            (*value).clone().unwrap()
        }
    }

    fn is_still_processing(completed: &mut Option<CompletionStatus>) -> bool {
        completed.is_none()
    }
}

pub fn create_data_with_completion<T>(data: T) -> (DataWithCompletion<T>, CompletionReceiver) {
    let status = Arc::new(CompletionCondition::new());
    let other_status = Arc::clone(&status);

    (
        DataWithCompletion {
            data,
            processed: false,
            sync: status,
        },
        CompletionReceiver { sync: other_status },
    )
}
