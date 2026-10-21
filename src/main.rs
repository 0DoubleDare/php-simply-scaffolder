use slint::ToSharedString;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError>{
    let app = AppWindow::new()?;

    let app_weak = app.as_weak();
    app.on_pick_target_folder({
        let app_weak = app_weak.clone();
        move || {
            let path = rfd::FileDialog::new().pick_folder().unwrap();

            path.to_string_lossy().to_shared_string()
        }

    });

    let _ = app.run();

    Ok(())
}
