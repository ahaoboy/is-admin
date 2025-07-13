#[cfg(windows)]
#[link(name = "shell32")]
unsafe extern "system" {
    fn IsUserAnAdmin() -> bool;
}

#[cfg(windows)]
pub fn is_admin() -> bool {
    unsafe { IsUserAnAdmin() }
}

#[cfg(unix)]
pub fn is_admin() -> bool {
    use libc::{geteuid, getuid};
    unsafe { getuid() == 0 || geteuid() == 0 }
}
