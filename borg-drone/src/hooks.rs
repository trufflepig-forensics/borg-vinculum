//! The hook execution is defined in this module

use std::ops::Sub;
use std::time::Instant;

use common::{HookStats, State};
use log::error;
use tokio::process::Command;

use crate::api::Api;

async fn execute_hook(command: &str) -> Result<HookStats, String> {
    let start = Instant::now();

    let cmd =
        shlex::split(command).ok_or(format!("Could not split given hook command: {command}"))?;
    let Some((cmd, args)) = cmd.split_first() else {
        return Err(format!("hook command was faulty"));
    };

    let out = Command::new(cmd)
        .args(args)
        .output()
        .await
        .map_err(|e| format!("Error spawning command: {e}"))?;

    if !out.status.success() {
        let stdout = String::from_utf8(out.stdout).unwrap_or("***Invalid stdout***".to_string());
        let stderr = String::from_utf8(out.stderr).unwrap_or("***Invalid stderr***".to_string());

        return Err(format!(
            "hook exited with status code {code}.\n\nStdout:\n{stdout}\n\nStderr:\n{stderr}",
            code = out.status.code().unwrap(),
        ));
    }

    let duration = Instant::now().sub(start);
    Ok(HookStats {
        duration: duration.as_secs(),
    })
}

/// Start running a hook.
///
/// This wrapper function will do the error handling and reporting to the vinculum
pub async fn run_hook(api: &Api, command: &str, hook_type: State) -> Result<HookStats, String> {
    let stats = match execute_hook(command).await {
        Ok(stats) => stats,
        Err(err) => {
            error!("Error in hook: {err}");
            if let Err(err) = api.send_error(&err, hook_type).await {
                error!("Error while sending error to vinculum: {err}");
            }
            return Err(err);
        }
    };

    Ok(stats)
}
