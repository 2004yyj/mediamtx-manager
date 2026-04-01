use crate::error::CoreError;

/// Unix: MediaMTX 프로세스에 SIGHUP 전송하여 설정 재적용
#[cfg(unix)]
pub fn send_reload_signal(pid: u32) -> Result<(), CoreError> {
    use nix::sys::signal::{kill, Signal};
    use nix::unistd::Pid;

    kill(Pid::from_raw(pid as i32), Signal::SIGHUP)
        .map_err(|e| CoreError::Process(format!("Failed to send SIGHUP: {e}")))
}

/// Windows: SIGHUP이 없으므로 프로세스 재시작이 필요
#[cfg(windows)]
pub fn send_reload_signal(_pid: u32) -> Result<(), CoreError> {
    Err(CoreError::Process(
        "Hot reload via signal is not supported on Windows. Use API-based config changes or restart the process.".into(),
    ))
}
