use runas_with_inherited_cmd as runas;

pub fn admin() -> anyhow::Result<()> {
    let mut binding = runas::Command::new(std::env::current_exe()?);
    let mut cmd = binding
        .args(std::env::args().skip(1).collect::<Vec<_>>().as_slice());


    
    #[cfg(target_os = "windows")]
    let show_flag = true;
    #[cfg(not(target_os = "windows"))]
    let show_flag = false;
    cmd = cmd.show(show_flag);
    
    match cmd.status()
    {
        Ok(_) => Ok(()),
        Err(_) => anyhow::bail!("failed"),
    }
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

#[cfg(unix)]
pub fn is_elevated() -> bool {
    nix::unistd::geteuid().is_root()
}
