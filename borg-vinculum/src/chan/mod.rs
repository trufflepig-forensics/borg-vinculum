use common::ErrorReport;
use log::{info, warn};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;

use crate::config::Config;
use crate::models::Drone;
use crate::modules::matrix::{MatrixApi, MatrixError};

/// Channel to the matrix notifier
pub type MatrixNotifierChan = Sender<(Drone, ErrorReport)>;

async fn perform_login(
    matrix: &mut MatrixApi,
    username: &str,
    password: &str,
    channel: &str,
) -> Result<(), String> {
    matrix
        .login(username, password)
        .await
        .map_err(|e| e.to_string())?;
    matrix.join_room(channel).await.map_err(|e| e.to_string())?;

    Ok(())
}

pub(crate) async fn start_matrix_notifier(
    config: &Config,
    mut matrix: MatrixApi,
) -> Result<MatrixNotifierChan, String> {
    let (tx, mut rx) = mpsc::channel::<(Drone, ErrorReport)>(16);

    let channel = config.matrix.channel.clone();
    let username = config.matrix.username.clone();
    let password = config.matrix.password.clone();

    info!("Logging in to matrix");
    perform_login(&mut matrix, &username, &password, &channel).await?;

    tokio::spawn(async move {
        while let Some((drone, report)) = rx.recv().await {
            let msg = format!(
                r#"🚨 The vinculum reports alarm for drone {drone_name}!
                
                {drone_name} failed in {state}
                
                {custom}{stderr}{stdout}"#,
                drone_name = drone.name.clone(),
                state = report.state,
                custom = report
                    .custom
                    .as_ref()
                    .map_or("".to_string(), |x| format!("Custom error:\n{x}\n\n")),
                stderr = report
                    .stderr
                    .as_ref()
                    .map_or("".to_string(), |x| format!("Stderr:\n{x}\n\n")),
                stdout = report
                    .stdout
                    .as_ref()
                    .map_or("".to_string(), |x| format!("Stdout:\n{x}")),
            );
            let formatted_msg = Some(format!(
                r#"<h4>🚨 The vinculum reports alarm for drone <font color="cyan">{drone_name}</font>!</h4>
                <p><font color="cyan">{drone_name}</font> failed in {state}</p>
                {custom}
                {stderr}
                {stdout}
            "#,
                drone_name = drone.name.clone(),
                state = report.state,
                custom = report.custom.as_ref().map_or("".to_string(), |x| format!(
                    "<p>Custom error:<br><code>{x}</code></p>"
                )),
                stderr = report.stderr.as_ref().map_or("".to_string(), |x| format!(
                    "<p>Stderr:<br><pre>{x}</pre></p>"
                )),
                stdout = report.stdout.as_ref().map_or("".to_string(), |x| format!(
                    "<p>Stdout:<br><pre>{x}</pre></p>"
                )),
            ));

            if let Err(err) = matrix.send_message(msg, formatted_msg, &channel).await {
                match err {
                    MatrixError::LoginFailed => {
                        if let Err(err) =
                            perform_login(&mut matrix, &username, &password, &channel).await
                        {
                            warn!("Error while performing re-login: {err}");
                        }
                    }
                    _ => warn!("{err}"),
                }
            }
        }
    });

    Ok(tx)
}
