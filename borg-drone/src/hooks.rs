//! The hook execution is defined in this module

use std::ops::Sub;
use std::time::Instant;

use common::{ErrorReport, HookStats, State};
use log::error;
use tokio::process::Command;

use crate::api::Api;

async fn execute_hook(command: &str, state: State) -> Result<HookStats, ErrorReport> {
    let start = Instant::now();

    let cmd = shlex::split(command).ok_or(ErrorReport {
        state,
        custom: Some(format!("Could not split given hook command: {command}")),
        stdout: None,
        stderr: None,
    })?;
    let Some((cmd, args)) = cmd.split_first() else {
        return Err(ErrorReport {
            state,
            custom: Some("hook command was faulty".to_string()),
            stdout: None,
            stderr: None,
        });
    };

    let out = Command::new(cmd)
        .args(args)
        .output()
        .await
        .map_err(|e| ErrorReport {
            state,
            custom: Some(format!("Error spawning command: {e}")),
            stdout: None,
            stderr: None,
        })?;

    if !out.status.success() {
        let stdout = String::from_utf8(out.stdout).unwrap_or("***Invalid stdout***".to_string());
        let stderr = String::from_utf8(out.stderr).unwrap_or("***Invalid stderr***".to_string());

        return Err(ErrorReport {
            state,
            custom: Some(format!(
                "Hook exited with status code: {code}",
                code = out.status.code().unwrap(),
            )),
            stdout: Some(stdout),
            stderr: Some(stderr),
        });
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
    let stats = match execute_hook(command, hook_type).await {
        Ok(stats) => stats,
        Err(err) => {
            error!("Error in hook: {err:?}");
            if let Err(err) = api.send_error(err.clone()).await {
                error!("Error while sending error to vinculum: {err}");
            }
            return Err(format!("{err:?}"));
        }
    };

    Ok(stats)
}
