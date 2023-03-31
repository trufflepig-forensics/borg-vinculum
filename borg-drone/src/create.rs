//! Creation of archives are defined here

use std::ops::Sub;
use std::time::Instant;

use borgbackup::asynchronous::CreateProgress;
use borgbackup::common::{CommonOptions, CompressionMode, CreateOptions};
use borgbackup::output::create::Create;
use byte_unit::Byte;
use common::{CreateStats, ErrorReport, State};
use log::{error, info};
use tokio::sync::mpsc;

use crate::api::Api;
use crate::config::Config;

async fn start_create(
    options: &CreateOptions,
    common_options: &CommonOptions,
) -> Result<Create, ErrorReport> {
    borgbackup::asynchronous::create(options, common_options)
        .await
        .map_err(|err| ErrorReport {
            state: State::Create,
            custom: Some(err.to_string()),
            stdout: None,
            stderr: None,
        })
}

async fn start_create_progress(
    options: &CreateOptions,
    common_options: &CommonOptions,
) -> Result<Create, ErrorReport> {
    let (tx, mut rx) = mpsc::channel(1);
    tokio::spawn(async move {
        while let Some(CreateProgress::Progress {
            original_size,
            compressed_size,
            deduplicated_size,
            path,
            ..
        }) = rx.recv().await
        {
            info!(
                "O: {o}, C: {c}, D: {d}, Path: {path}",
                o = Byte::from(original_size as u128).get_appropriate_unit(false),
                c = Byte::from(compressed_size as u128).get_appropriate_unit(false),
                d = Byte::from(deduplicated_size as u128).get_appropriate_unit(false),
            )
        }
    });

    borgbackup::asynchronous::create_progress(options, common_options, tx)
        .await
        .map_err(|err| ErrorReport {
            state: State::Create,
            custom: Some(err.to_string()),
            stdout: None,
            stderr: None,
        })
}

/// Create a backup using the settings from [Config].
pub async fn create(config: &Config, progress: bool) -> Result<CreateStats, ErrorReport> {
    let start = Instant::now();

    let common_options = CommonOptions {
        remote_path: config.borg.remote_path.clone(),
        ..CommonOptions::default()
    };

    let options = CreateOptions {
        repository: config.borg.repository.clone(),
        archive: "{utcnow}".to_string(),
        passphrase: Some(config.borg.passphrase.clone()),
        comment: None,
        compression: Some(CompressionMode::Lz4),
        paths: vec![],
        exclude_caches: false,
        patterns: vec![],
        pattern_file: Some(config.borg.pattern_file_path.clone()),
        excludes: vec![],
        exclude_file: None,
        numeric_ids: false,
        sparse: true,
        read_special: false,
        no_xattrs: false,
        no_acls: false,
        no_flags: false,
    };

    let stats = if progress {
        start_create_progress(&options, &common_options).await?
    } else {
        start_create(&options, &common_options).await?
    };

    let duration = Instant::now().sub(start);

    Ok(CreateStats {
        original_size: stats.archive.stats.original_size,
        compressed_size: stats.archive.stats.compressed_size,
        deduplicated_size: stats.archive.stats.deduplicated_size,
        nfiles: stats.archive.stats.nfiles,
        duration: duration.as_secs(),
    })
}

/// Wrapper for [create].
///
/// This will do the error handling for the create call.
pub async fn run_create(api: &Api, config: &Config, progress: bool) -> Result<CreateStats, String> {
    let stats = match create(config, progress).await {
        Ok(stats) => stats,
        Err(err) => {
            error!("Error while creating archive: {err:#?}");
            if let Err(err) = api.send_error(err.clone()).await {
                error!("Error while sending error to vinculum: {err}");
            }
            return Err(format!("{err:?}"));
        }
    };

    Ok(stats)
}
