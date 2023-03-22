//! Creation of archives are defined here

use std::time::Instant;

use borgbackup::common::{CommonOptions, CompressionMode, CreateOptions};
use byte_unit::Byte;
use log::info;

use crate::config::Config;

/// Create a backup using the settings from [Config].
pub async fn create(config: Config) -> Result<(), String> {
    let common_options = CommonOptions {
        remote_path: config.borg.remote_path,
        ..CommonOptions::default()
    };

    let options = CreateOptions {
        repository: config.borg.repository,
        archive: "{now}".to_string(),
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

    info!("Starting to create the archive");
    let start = Instant::now();

    let stats = borgbackup::asynchronous::create(&options, &common_options)
        .await
        .map_err(|err| err.to_string())?;

    let duration = Instant::now().duration_since(start);
    info!(
        "Archive created, O: {o}, C: {c}, D: {d}, took {t} seconds",
        o = Byte::from(stats.archive.stats.original_size as u128).get_appropriate_unit(false),
        c = Byte::from(stats.archive.stats.compressed_size as u128).get_appropriate_unit(false),
        d = Byte::from(stats.archive.stats.deduplicated_size as u128).get_appropriate_unit(false),
        t = duration.as_secs()
    );

    Ok(())
}
