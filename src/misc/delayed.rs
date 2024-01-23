use std::time::{Duration, Instant};

pub struct Delayed<T> {
    data: Option<T>,
    last_update: Instant,
    timeout: Option<Duration>,
}

pub enum DelayedResult<T> {
    Ok(T),
    Undefined,
    Outdated,
}

impl<T> Delayed<T> {
    pub fn new() -> Self {
        Self {
            data: None,
            last_update: Instant::now(),
            timeout: None,
        }
    }

    pub fn get(&self) -> DelayedResult<&T> {
        if self.data.is_none() {
            return DelayedResult::Undefined;
        }

        let now = Instant::now();
        if let Some(timeout) = self.timeout {
            if self.last_update + timeout < now {
                return DelayedResult::Outdated;
            }
        }

        DelayedResult::Ok(self.data.as_ref().unwrap())
    }

    pub fn with_timeout(self, timeout: Duration) -> Self {
        Self {
            timeout: Some(timeout),
            ..self
        }
    }

    pub fn with_data(self, data: T) -> Self {
        Self {
            data: Some(data),
            ..self
        }
    }
}
