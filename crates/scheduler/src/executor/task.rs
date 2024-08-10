use std::fmt::Debug;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct TaskId(pub usize);

pub struct Task {
    id: TaskId,
    job: Option<Box<dyn FnMut() + Send + Sync + 'static>>,
}

impl TaskId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}

impl Task {
    pub fn new(id: TaskId) -> Self {
        Self { id, job: None }
    }

    pub fn set_handler<F>(&mut self, handler: F)
    where
        F: FnMut() + Send + Sync + 'static,
    {
        self.job.replace(Box::new(handler));
    }

    pub fn id(&self) -> TaskId {
        self.id
    }

    pub(crate) fn run(&self) -> Result<(), ()> {
        Ok(())
    }

}
