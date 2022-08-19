#[derive(Debug, PartialEq)]
pub enum Error {
    ResetSignal(u32),
    CloseHandle(u32),
    CreateEvent(u32),
    SignalEvent(u32),
    WaitSynchronization(u32),
    Moudle(MOUDLE),
}

#[derive(Debug, PartialEq)]
pub enum MOUDLE {
    TimedOut = 117,
    Cancelled = 118,
}

pub type Result<T> = core::result::Result<T, Error>;
