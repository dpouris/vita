use crate::Task;

pub mod task;

pub(crate) struct Executor<'e> {
    jobs: Vec<&'e Task>,
}

impl<'e> Default for Executor<'e> {
    fn default() -> Self {
        Self {
            jobs: Vec::new()
        }
    }
}

impl<'e> Executor<'e> {
    pub fn new() -> Self {
        Self::default()
    }
}