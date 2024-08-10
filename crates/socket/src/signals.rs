pub enum Signal {
    CtrlC,
}

pub enum Action {
    TerminateExit(i32),
    Terminate,
}
