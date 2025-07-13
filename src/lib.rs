// #[cfg(windows)]
// #[link(name = "shell32")]
// unsafe extern "system" {
//     fn IsUserAnAdmin() -> bool;
// }

// #[cfg(windows)]
// pub fn is_admin() -> bool {
//     unsafe { IsUserAnAdmin() }
// }

#[cfg(windows)]
pub fn is_admin() -> bool {
    let shell = "[bool]([System.Security.Principal.WindowsPrincipal][System.Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([System.Security.Principal.WindowsBuiltInRole]::Administrator)";
    let output = std::process::Command::new("powershell")
        .args(["-c", shell])
        .output()
        .expect("Failed to execute powershell command");
    String::from_utf8(output.stdout).unwrap_or_default().trim() == "True"
}

#[cfg(unix)]
pub fn is_admin() -> bool {
    use libc::{geteuid, getuid};
    unsafe { getuid() == 0 || geteuid() == 0 }
}
