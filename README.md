# self_runas

Cross‑platform lib to elevate current process privilege to administrator/root.

Usage
```rust
fn main() -> anyhow::Result<()> {
    
    if !self_runas::is_elevated() {
        // relaunch elevated
        self_runas::admin()?;         
        return Ok(());
    }

    // privileged work here
    Ok(())
}
```

Install
- Cargo.toml:
  [dependencies]
  self_runas = "0.1"

Notes
- Propagates current args to elevated process.
- Windows shows UAC by default; hides on non‑Windows.
- Returns Err on failure to elevate.

License
MIT OR Apache-2.0