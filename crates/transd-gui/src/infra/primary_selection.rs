//! Utilities for PRIMARY selection on Linux.
//!
//! PERF: Here we use external commands to read PRIMARY, which is not very efficient. A better way
//! would be to talk to the X11/Wayland compositor directly, but that requires more complex code and
//! dependencies.

use std::{
    env, fs,
    path::{Path, PathBuf},
    sync::LazyLock,
};
use tokio::process::Command;

fn which(cmd: &str) -> Option<PathBuf> {
    static PATH: LazyLock<Vec<PathBuf>> = LazyLock::new(|| {
        env::var_os("PATH")
            .map(|paths| env::split_paths(&paths).collect())
            .unwrap_or_default()
    });

    PATH.iter()
        .map(|p| p.join(cmd))
        .find(|p| fs::metadata(p).map(|m| m.is_file()).unwrap_or(false))
}

async fn run_capture(cmd: &Path, args: &[&str]) -> Result<String, String> {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .await
        .map_err(|e| format!("failed to spawn {}: {e}", cmd.display()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stderr = stderr.trim();
        if stderr.is_empty() {
            return Err(format!("{} exited with {output:?}", cmd.display()));
        }
        return Err(format!("{} failed: {stderr}", cmd.display()));
    }

    String::from_utf8(output.stdout)
        .map(|s| s.trim_end_matches(['\n', '\r']).to_string())
        .map_err(|e| format!("{} output is not utf8: {e}", cmd.display()))
}

/// Read PRIMARY selection without needing a focused GUI window.
///
/// On Wayland, GUI toolkits often require a focused surface to access PRIMARY.
/// `wl-paste --primary` talks to the compositor directly and works in background.
pub async fn read_primary_best_effort() -> Result<Option<String>, String> {
    static IS_WAYLAND: LazyLock<bool> = LazyLock::new(|| env::var_os("WAYLAND_DISPLAY").is_some());
    static IS_X11: LazyLock<bool> = LazyLock::new(|| env::var_os("DISPLAY").is_some());

    static WL_PASTE_PATH: LazyLock<Option<PathBuf>> = LazyLock::new(|| which("wl-paste"));
    static XCLIP_PATH: LazyLock<Option<PathBuf>> = LazyLock::new(|| which("xclip"));
    static XSEL_PATH: LazyLock<Option<PathBuf>> = LazyLock::new(|| which("xsel"));

    // Wayland: prefer wl-paste.
    if *IS_WAYLAND && let Some(cmd) = WL_PASTE_PATH.as_ref() {
        // `-n/--no-newline`: do not append newline.
        let mut text = run_capture(cmd, &["--primary", "--no-newline"]).await;
        if text.is_err() {
            // Some versions of wl-paste do not support --no-newline, so try again without it.
            text = run_capture(cmd, &["--primary"]).await;
        }

        let text = text?;
        return Ok((!text.is_empty()).then_some(text));
    }

    // X11: xclip / xsel.
    if *IS_X11 {
        if let Some(cmd) = XCLIP_PATH.as_ref() {
            let text = run_capture(cmd, &["-o", "-selection", "primary"]).await?;
            return Ok((!text.is_empty()).then_some(text));
        }
        if let Some(cmd) = XSEL_PATH.as_ref() {
            let text = run_capture(cmd, &["-o", "-p"]).await?;
            return Ok((!text.is_empty()).then_some(text));
        }
    }

    Ok(None)
}
