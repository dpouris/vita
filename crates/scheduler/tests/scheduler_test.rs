use scheduler::{schedule_every, time::AsTimeSpan};
use scheduler::time::WeekDay::{Monday, Tuesday};

#[test]
fn test_adding_schedules() -> Result<(), ()> {
    let threads = std::thread::available_parallelism().unwrap();

    println!("Available threads: {}", threads);
    let mut scheduler = schedule_every!(
        1.year().at("12:32")? => {
            println!("Every year at 12:32 PM!")
        },

        2.hour() => {
            println!("Every 2 hours!")
        },

        2.day().and(6.hour()) => {
            println!("Every 2 days and 6 hours!")
        },

        1.week() => {
            println!("Every week!")
        },

        1.week(), 1.day() => {
            println!("Every week and every day!!")
        },

        Monday.midnight() => {
            println!("Every Monday at midnight!")
        },

        Tuesday.afternoon(), 3.day().at("3:45")?.am()? => {
            println!("Every Tuesday afternoon AND every 3 days at 3:45 AM!")
        },

        5.month() => {
            println!("Every month!")
        },

        2.year() => {
            println!("Every 2 years!")
        }
    );
    
    for sc in scheduler {
        
    }
    
    scheduler.every(1.hour());

    assert_eq!(1, 1);
    Ok(())
}
