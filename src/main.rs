use slint::{SharedString, ToSharedString};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError>{
    let app = AppWindow::new()?;
    let php_project_template = include_dir::include_dir!("$CARGO_MANIFEST_DIR/templates/php-project-template");

    let app_weak = app.as_weak();
    app.on_pick_target_folder({
        let app_weak = app_weak.clone();
        move || {
            if let Some(cur_path) = rfd::FileDialog::new().pick_file() {
                if let Some(ui) = app_weak.upgrade() {
                    let path = cur_path.to_string_lossy().to_shared_string();
                    ui.set_target_install_path(path);
                }
            }
        }
    });

    app.on_start_install_template({
        let app_weak = app_weak.clone();
        move || {

        }
    });

    let _ = app.run();

    Ok(())
}
