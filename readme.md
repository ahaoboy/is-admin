Check if the current user is admin, use `powershell` on windows and `libc::getuid/libc::geteuid` on unix

```rust
fn main() {
    println!("is_admin: {}", is_admin::is_admin())
}
```