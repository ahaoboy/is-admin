import { spawnSync } from 'child_process'

export function isAdminWindows() {
  const cmd =
    `[bool]([System.Security.Principal.WindowsPrincipal][System.Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([System.Security.Principal.WindowsBuiltInRole]::Administrator)`
  const output = spawnSync('powershell', ['-c', cmd]).stdout.toString().trim()
  return output === 'True'
}

export function isAdminUnix() {
  return process.getuid?.() === 0
}

export function isAdmin(): boolean {
  return process.platform === 'win32' ? isAdminWindows() : isAdminUnix()
}
