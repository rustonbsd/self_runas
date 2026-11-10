use runas_with_inherited_cmd as runas;
use std::ffi::OsString;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to get current exe path")]
    CurrentPath(#[source] std::io::Error),

    #[error("Run as root, could not elevate privileges automatically: ")]
    ExecAsAdmin(#[source] std::io::Error),
}

pub fn admin() -> Result<(), Error> {
    if is_elevated() {
        return Ok(());
    }

    let exe = std::env::current_exe().map_err(Error::CurrentPath)?;
    let args: Vec<OsString> = std::env::args_os().skip(1).collect();

    let status = runas::Command::new(exe)
        .args(&args)
        .force_prompt(false)
        .show(cfg!(windows))
        .status()
        .map_err(Error::ExecAsAdmin)?;
    std::process::exit(status.code().unwrap_or(1));
}

#[cfg(unix)]
pub fn is_elevated() -> bool {
    nix::unistd::geteuid().is_root()
}

#[cfg(windows)]
pub fn is_elevated() -> bool {
    use winapi::shared::minwindef::DWORD;
    use winapi::um::handleapi::CloseHandle;
    use winapi::um::processthreadsapi::{GetCurrentProcess, OpenProcessToken};
    use winapi::um::securitybaseapi::GetTokenInformation;
    use winapi::um::winnt::{TOKEN_ELEVATION, TOKEN_QUERY, TokenElevation};

    unsafe {
        let process = GetCurrentProcess();
        let mut token = std::ptr::null_mut();
        if OpenProcessToken(process, TOKEN_QUERY, &mut token) == 0 {
            return false;
        }
        let mut elev: TOKEN_ELEVATION = std::mem::zeroed();
        let mut ret_len: DWORD = 0;
        let ok = GetTokenInformation(
            token,
            TokenElevation,
            &mut elev as *mut _ as *mut _,
            std::mem::size_of::<TOKEN_ELEVATION>() as DWORD,
            &mut ret_len,
        );
        CloseHandle(token);
        if ok == 0 {
            return false;
        }
        elev.TokenIsElevated != 0
    }
}

#[cfg(not(any(unix, windows)))]
pub fn is_elevated() -> bool {
    false
}
