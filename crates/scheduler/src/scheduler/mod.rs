use std::ops::Deref;

pub use macros::*;
pub use schedule::*;

pub use crate::executor::task::*;
use crate::time::TimeSpan;

mod schedule;
#[macro_use]
mod macros;

pub struct Scheduler {
    schedules: Vec<Schedule>,
    pk: usize,
    // _marker: &'s PhantomData<T>,
}

impl Default for Scheduler {
    fn default() -> Self {
        Self {
            schedules: Vec::new(),
            pk: 0,
            // _marker: &PhantomData,
        }
    }
}

impl Scheduler {
    pub fn new() -> Self {
        Self::default()
    }

    // TODO: the with_tz method needs to parse the _tz &str to a valid timezone. If the timezone is not valid then return an error.
    // Need to check if this approach should change from a &str to a timezone enum for more predictable results
    pub fn with_tz(_tz: &str) -> Result<Self, ()> {
        unimplemented!()
    }

    pub fn every(&mut self, ts: TimeSpan) -> &mut Schedule {
        let task_id = self.next_task_id();
        let schedule = Schedule::new(ts, task_id);
        self.add_schedule(task_id, schedule)
    }

    fn stop_schedule(&self) -> Option<Schedule> {
        todo!()
    }

    /// Refreshes the schedules to be up-to-date, checks if the `run_on` [Timestamp]s are expired and updates them if necessary. It then sorts
    /// the `schedules` member in `Scheduler` by ascending order of the `run_on` member
    pub fn refresh(&mut self) {
        self.schedules
            .iter_mut()
            .filter_map(|sc| sc.has_expired().then(|| sc.next_run_on()));
        self.schedules.sort_by(|ref sc1, ref sc2| sc1.run_on.cmp(&sc2.run_on));
    }

    fn add_schedule(&mut self, task_id: TaskId, schedule: Schedule) -> &mut Schedule {
        self.schedules.insert(task_id.0, schedule);
        self.schedules.get_mut(task_id.0).unwrap() // we get schedule here because then the schedules may become unordered due to the .refresh() call
    }

    fn next_task_id(&mut self) -> TaskId {
        let id = TaskId(self.pk);
        self.pk += 1;
        id
    }
}

// pub struct SchedulerIter<'s> {
//     inner:  &'s Scheduler,
//     cur: usize,
// }
//
//
// impl<'s> Iterator for SchedulerIter<'s> {
//     type Item = &'s Schedule;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.cur > self.inner.schedules.len() - 1 {
//             // if we want the iterator to start over we set the current_schedule to 0 -> self.current_schedule = 0;
//             return None;
//         }
//
//         let schedule = Some(&self.inner.schedules[self.cur]);
//         self.cur += 1;
//
//         schedule
//     }
//
//     // fn size_hint(&self) -> (usize, Option<usize>) {}
// }
//
// impl<'s> IntoIterator for &'s Scheduler {
//     type Item = &'s Schedule;
//
//     type IntoIter = SchedulerIter<'s>;
//     fn into_iter(self) -> Self::IntoIter {
//         SchedulerIter {
//             inner: self,
//             cur: 0,
//         }
//     }
// }
