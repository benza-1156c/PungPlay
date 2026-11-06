pub fn get_ytdlp_download_info() -> (&'static str, &'static str) {
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    return (
        "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe",
        "yt-dlp.exe",
    );

    #[cfg(all(target_os = "windows", target_arch = "aarch64"))]
    return (
        "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_arm64.exe",
        "yt-dlp.exe",
    );

    #[cfg(target_os = "macos")]
    return (
        "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_macos",
        "yt-dlp",
    );

    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    return (
        "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_linux",
        "yt-dlp",
    );

    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    return (
        "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_linux_aarch64",
        "yt-dlp",
    );

    #[cfg(not(any(
        all(target_os = "windows", target_arch = "x86_64"),
        all(target_os = "windows", target_arch = "aarch64"),
        target_os = "macos",
        all(target_os = "linux", target_arch = "x86_64"),
        all(target_os = "linux", target_arch = "aarch64")
    )))]
    compile_error!("Unsupported OS/Architecture for yt-dlp download");
}
