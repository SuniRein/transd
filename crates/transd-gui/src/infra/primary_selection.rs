//! Utilities for PRIMARY selection on Linux.
//!
//! PERF: Here we use external commands to read PRIMARY, which is not very efficient. A better way
//! would be to talk to the X11/Wayland compositor directly, but that requires more complex code and
//! dependencies.

use std::env;
use tokio::process::Command;

async fn which(cmd: &str) -> bool {
    Command::new("sh")
        .arg("-lc")
        .arg(format!("command -v {cmd} >/dev/null 2>&1"))
        .status()
        .await
        .is_ok_and(|s| s.success())
}

async fn run_capture(cmd: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .await
        .map_err(|e| format!("failed to spawn {cmd}: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stderr = stderr.trim();
        if stderr.is_empty() {
            return Err(format!("{cmd} exited with {output:?}"));
        }
        return Err(format!("{cmd} failed: {stderr}"));
    }

    String::from_utf8(output.stdout)
        .map(|s| s.trim_end_matches(['\n', '\r']).to_string())
        .map_err(|e| format!("{cmd} output is not utf8: {e}"))
}

/// Read PRIMARY selection without needing a focused GUI window.
///
/// On Wayland, GUI toolkits often require a focused surface to access PRIMARY.
/// `wl-paste --primary` talks to the compositor directly and works in background.
pub async fn read_primary_best_effort() -> Result<Option<String>, String> {
    // Wayland: prefer wl-paste.
    if env::var_os("WAYLAND_DISPLAY").is_some() && which("wl-paste").await {
        // `-n/--no-newline`: do not append newline.
        let mut text = run_capture("wl-paste", &["--primary", "--no-newline"]).await;
        if text.is_err() {
            // Some versions of wl-paste do not support --no-newline, so try again without it.
            text = run_capture("wl-paste", &["--primary"]).await;
        }

        let text = text?.trim().to_string();
        return Ok((!text.is_empty()).then_some(text));
    }

    // X11: xclip / xsel.
    if env::var_os("DISPLAY").is_some() {
        if which("xclip").await {
            let text = run_capture("xclip", &["-o", "-selection", "primary"]).await?;
            let text = text.trim().to_string();
            return Ok((!text.is_empty()).then_some(text));
        }
        if which("xsel").await {
            let text = run_capture("xsel", &["-o", "-p"]).await?;
            let text = text.trim().to_string();
            return Ok((!text.is_empty()).then_some(text));
        }
    }

    Ok(None)
}
