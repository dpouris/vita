use crate::{Task, TaskId};
use crate::time::TimeSpan;
use crate::time::timestamp::Timestamp;

pub struct Schedule {
    pub(crate) run_on: Timestamp,
    pub(crate) task: Task,
    pub(crate) ts: TimeSpan,
}

impl Schedule {
    pub fn new(mut ts: TimeSpan, task_id: TaskId) -> Self {
        let run_on = ts.next_run_on();
        Self {
            run_on: run_on,
            task: Task::new(task_id),
            ts,
        }
    }

    pub fn run_when_ready(&self) -> Result<(), ()> {
        println!("Running {}...", self.task.id().0);
        Ok(())
    }

    /// Returns `true` if `has_expired` is true, else returns `false`.
    pub fn has_expired(&mut self) -> bool {
        self.run_on <= Timestamp::now()
    }

    pub fn perform<F>(&mut self, task: F) -> TaskId
    where
        F: FnMut() + Send + Sync + 'static,
    {
        //before we allocate and do all that we must check the run_on field for .is_some() and if not return error
        self.task.set_handler(Box::new(task));
        self.task.id()
    }

    pub fn perform_task(&self, _p0: ()) -> Result<(), ()> {
        todo!()
    }

    pub fn start_now(&mut self) -> &mut Self {
        // self.at()
        todo!()
    }

    pub(crate) fn next_run_on(&mut self) -> &Timestamp {
        let run_on = self.ts.next_run_on();
        self.run_on = run_on;
        &self.run_on
    }
}
