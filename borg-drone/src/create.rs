//! Creation of archives are defined here

use std::ops::Sub;

use borgbackup::asynchronous::CreateProgress;
use borgbackup::common::{CommonOptions, CompressionMode, CreateOptions};
use byte_unit::Byte;
use chrono::Utc;
use log::info;
use tokio::sync::mpsc;

use crate::config::Config;

/// Create a backup using the settings from [Config].
pub async fn create(config: Config) -> Result<(), String> {
    let common_options = CommonOptions {
        remote_path: config.borg.remote_path,
        ..CommonOptions::default()
    };

    let options = CreateOptions {
        repository: config.borg.repository,
        archive: "{utcnow}".to_string(),
        passphrase: Some(config.borg.passphrase),
        comment: None,
        compression: Some(CompressionMode::Lz4),
        paths: vec![],
        exclude_caches: false,
        patterns: vec![],
        pattern_file: Some(config.borg.pattern_file_path),
        excludes: vec![],
        exclude_file: None,
        numeric_ids: false,
        sparse: true,
        read_special: false,
        no_xattrs: false,
        no_acls: false,
        no_flags: false,
    };

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

    info!("Starting to create the archive");
    let start = Utc::now();

    let stats = borgbackup::asynchronous::create_progress(&options, &common_options, tx)
        .await
        .map_err(|err| err.to_string())?;

    let duration = Utc::now().sub(start);
    info!(
        "Archive created, O: {o}, C: {c}, D: {d}, took {duration}",
        o = Byte::from(stats.archive.stats.original_size as u128).get_appropriate_unit(false),
        c = Byte::from(stats.archive.stats.compressed_size as u128).get_appropriate_unit(false),
        d = Byte::from(stats.archive.stats.deduplicated_size as u128).get_appropriate_unit(false),
    );

    Ok(())
}
