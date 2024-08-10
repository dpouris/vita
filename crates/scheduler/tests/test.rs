use scheduler::Scheduler;
use scheduler::task::{Job, TaskId};
use scheduler::time::AsTimeSpan;

#[test]
fn test() -> Result<(), ()> {
    let mut scheduler = Scheduler::with_tz("UTC+2").expect("cannot set timezone to utc+2");

    // TimeSpan -> Weekday(WeekDay), Monthday(1-31), Year(u8), Month(u8), Day(u16), Hour(u16), Minute(u32), Sec(u32)
    // WeekDay -> Sunday, Monday, Tuesday, Wednesday, Thursday, Friday, Saturday

    // .every(TimeSpan) stores TimeSpan temporarily on a semi-complete Schedule struct.
    // Upon the .perform() or .perform_task() calls it stores the Task and completes the Schedule struct.
    // It then parses the TimeSpan struct to validate its validity.
    // If the parse fails, then a detailed error is returned
    // Scheduler.every(TimeSpan) -> &mut Schedule.perform(FnMut()) -> Result<Task, Error>
    let Ok(task) = scheduler.every(1.day().at("10:00")?.am()?).perform(|| {
        println!("hi");
    }) else {
        return Err(());
    };

    scheduler.every(Monday.midnight());
    scheduler.every(Monday.at("10:00")?);
    // Scheduler.every(TimeSpan) -> &mut Schedule.perform_task(Task) -> Result<(), Error>
    let Ok(_) = scheduler
        .every(2.hour() + 34.minute() + 20.second())
        .perform_task(task)
    else {
        todo!()
    };

    // &mut Schedule.start_now() implies that the TimeSpan should take effect starting from now
    // (now means at the initialization of the schedule).
    // By default, the schedule will take effect at the next available time slot starting from today@12:00AM.
    // In the case of .every(Sec(20)), since there's no concrete time or date provided, the schedule will first run 20 seconds
    // starting the count from the time of initialization.
    let Ok(_) = scheduler.every(20.second()).start_now().perform_task(task) else {
        todo!()
    };

    // Scheduler.from_tasks(TaskId) -> Option<&Task>.map(|task| task.run())
    let task_id = TaskId::new(0);
    scheduler.tasks(task_id).map(|task| {
        task.run()
            .map_err(|_err| eprint!("Task with id {:?} not found", task_id))
    });

    Ok(())
}
