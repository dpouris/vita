enum ProcessType {
    // The parent process holds the child process id
    Parent(i32),
    Child,
}

unsafe fn handle_child_process() -> Result<(), Option<String>> {
    let gid = match libc::setsid() {
        -1 => Err(Some(
            "process already a group leader or process ID matches foreign process GID".into(),
        )),
        gid => Ok(gid),
    }?;

    println!("process GID is: {gid}");

    match create_child_process() {
        Ok(ProcessType::Child) => Ok(()),
        Ok(ProcessType::Parent(pid)) => {
            println!("new PID is: {pid}");
            Err(None)
        }
        Err(error_msg) => Err(Some(error_msg.into())),
    }
}

fn create_child_process() -> Result<ProcessType, &'static str> {
    unsafe {
        match libc::fork() {
            -1 => Err("could not fork process"),
            0 => Ok(ProcessType::Child),
            child_id => Ok(ProcessType::Parent(child_id)),
        }
    }
}

pub fn start_daemon() -> Result<(), Option<String>> {
    match create_child_process() {
        Ok(ProcessType::Child) => Ok(()),
        Ok(ProcessType::Parent(_)) => Err(None),
        Err(error_msg) => Err(Some(format!("could not start daemon: {error_msg}"))),
    }?;

    unsafe {
        handle_child_process()?;
    }

    Ok(())
}
