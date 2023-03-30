//! The hook execution is defined in this module

use std::ops::Sub;
use std::time::Instant;

use common::{HookType, Stats};
use log::{error, warn};
use tokio::process::Command;

use crate::api::Api;
use crate::config::Config;

async fn run_hook(command: &str, hook_type: HookType) -> Result<Stats, String> {
    let start = Instant::now();

    let cmd = shlex::split(command).ok_or(format!(
        "Could not split given {hook_type} hook command: {command}"
    ))?;
    let Some((cmd, args)) = cmd.split_first() else {
        return Err(format!("{hook_type} hook command was faulty"));
    };

    let out = Command::new(cmd)
        .args(args)
        .output()
        .await
        .map_err(|e| format!("Error spawning {hook_type} command: {e}"))?;

    if !out.status.success() {
        let stdout = String::from_utf8(out.stdout).unwrap_or("***Invalid stdout***".to_string());
        let stderr = String::from_utf8(out.stderr).unwrap_or("***Invalid stderr***".to_string());

        return Err(format!(
            "{hook_type} hook exited with status code {code}.\n\nStdout:\n{stdout}\n\nStderr:\n{stderr}",
            code = out.status.code().unwrap(),
        ));
    }

    let duration = Instant::now().sub(start);
    Ok(Stats::Hook {
        duration,
        hook_type,
    })
}

/// Start running the pre hook.
///
/// This wrapper function will do the error handling and reporting to the vinculum
pub async fn run_pre_hook(api: &Api, config: &Config) -> Result<(), String> {
    match run_hook(&config.pre_hook, HookType::Pre).await {
        Ok(stats) => {
            if let Err(err) = api.send_stats(stats).await {
                warn!("Could not send stats from pre hook to vinculum: {err}");
            }
        }
        Err(err) => {
            error!("Error in pre hook: {err}");
            if let Err(err) = api.send_error(&err).await {
                error!("Error while sending error to vinculum: {err}");
            }
            return Err(err);
        }
    }

    Ok(())
}

/// Start running the post hook.
///
/// This wrapper function will do the error handling and reporting to the vinculum
pub async fn run_post_hook(api: &Api, config: &Config) -> Result<(), String> {
    match run_hook(&config.post_hook, HookType::Post).await {
        Ok(stats) => {
            if let Err(err) = api.send_stats(stats).await {
                warn!("Could not send stats from post hook to vinculum: {err}");
            }
        }
        Err(err) => {
            error!("Error in post hook: {err}");
            if let Err(err) = api.send_error(&err).await {
                error!("Error while sending error to vinculum: {err}");
            }
            return Err(err);
        }
    }

    Ok(())
}
