#[macro_export]
macro_rules! schedule_every {
    (
        $(
            $($ts:expr),+ $(,)? => {
                $task:expr
            }
        ),+ $(,)?
    ) => {
        {
            let mut scheduler = $crate::Scheduler::new();
            $(
                $(
                    scheduler.every($ts).perform(|| {
                        $task;
                    });
                )+
            )+;

            scheduler
        }
    };
}
