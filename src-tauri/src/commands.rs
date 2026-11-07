#[macro_export]
macro_rules! app_commands {
    () => {
        tauri_specta::collect_commands![
            $crate::features::auth::commands::login_youtube,
            $crate::features::auth::commands::is_authenticated,
            $crate::features::auth::commands::list_profiles,
            $crate::features::auth::commands::switch_profile,
            $crate::features::ytdlp::commands::check_ytdlp_ready,
            $crate::features::ytdlp::commands::trigger_ytdlp_download,
        ]
    };
}
