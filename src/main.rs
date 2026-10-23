#![windows_subsystem = "windows"]

use std::{path::PathBuf, str::FromStr};

use php_generator::model::{extract_archive, fetch_archive, open_path_in_editor, structs::StateMessage};
use reqwest::Url;
use slint::{SharedString, ToSharedString};
use tracing::Level;
use zip::ZipArchive;

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
    tracing_subscriber::fmt()
        .compact()
        .with_level(true)
        .with_max_level(Level::DEBUG)
        .init();

    let app = AppWindow::new()?;

    let app_weak = app.as_weak();
    app.on_pick_target_folder({
        let weak = app_weak.clone();
        move || {
            let async_weak = weak.clone();
            tokio::spawn(async move {
                match rfd::FileDialog::new().pick_folder() {
                    Some(current_path) => {
                        let _ = slint::invoke_from_event_loop(move || {
                            if let Some(ui) = async_weak.upgrade() {
                                let path = current_path.to_string_lossy().to_shared_string();
                                tracing::info!("{}", path);
                                ui.set_target_install_path(path);
                            }
                        });

                    },
                    None => {
                        let _ = slint::invoke_from_event_loop(move || {
                            if let Some(ui) = async_weak.upgrade() {
                                ui.set_state_message(StateMessage::ErrorFailedToPickFolder.to_shared_string());
                            }
                        });
                    },
                }
            });

        }
    });

    app.on_start_install_template({
        let weak = app_weak.clone();
        move || {
            let mut url: String = "".to_string();
            let mut path: String = "".to_string();
            if let Some(ui) = weak.upgrade() {
                url = ui.get_template_archive_url().to_string();
                path = ui.get_target_install_path().to_string();

                ui.set_download_state(DownloadState::Download);
                ui.set_state_message(StateMessage::DownloadStart.to_shared_string());
            };

            let async_weak = weak.clone();
            tokio::spawn(async move {

                let ui_update_status = async_weak.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(ui) = ui_update_status.upgrade() {
                        ui.set_state_message(StateMessage::DownloadParseUrl.to_shared_string());
                    }
                });

                if let Ok(url) = Url::from_str(&url) {

                    let ui_update_status = async_weak.clone();
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(ui) = ui_update_status.upgrade() {
                            ui.set_state_message(StateMessage::DownloadParseLocalPath.to_shared_string());
                        }
                    });

                    if let Ok(mut target_path) = PathBuf::from_str(&path) {

                        if !target_path.is_dir() {
                            target_path = target_path.join("/");
                        }

                        let ui_update_status = async_weak.clone();
                        let _ = slint::invoke_from_event_loop(move || {
                            if let Some(ui) = ui_update_status.upgrade() {
                                ui.set_state_message(StateMessage::DownloadFetchArchive.to_shared_string());
                            }
                        });
                        if let Ok(result_path_archive) = fetch_archive(&target_path, &url).await {

                            let ui_update_status = async_weak.clone();
                            let _ = slint::invoke_from_event_loop(move || {
                                if let Some(ui) = ui_update_status.upgrade() {
                                    ui.set_state_message(StateMessage::DownloadExtractArchive.to_shared_string());
                                }
                            });
                            match extract_archive(&result_path_archive, &target_path).await {
                                Ok(_) => {
                                    let ui_update_status = async_weak.clone();
                                    let _ = slint::invoke_from_event_loop(move || {
                                        if let Some(ui) = ui_update_status.upgrade() {
                                            ui.set_download_state(DownloadState::Idle);
                                            ui.set_state_message(StateMessage::Done.to_shared_string());
                                        }
                                    });
                                    open_path_in_editor(&target_path);
                                },
                                Err(err) => {
                                    tracing::info!("Ошибка загрузки");
                                }
                            };
                        }
                    }
                }
            });
        }
    });

    app.on_toggle_url_line_edit({
        let app_weak = app_weak.clone();
        move |is_enabled| {
            if let Some(ui) = app_weak.upgrade() {
                ui.set_url_edit_is_enabled(!is_enabled);
            }
        }
    });

    let _ = app.run();

    Ok(())
}
