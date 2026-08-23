#!/bin/bash

mkdir -p src-tauri/binaries
cd src-tauri/binaries

curl -L -o yt-dlp-x86_64-pc-windows-msvc.exe https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe

curl -L -o yt-dlp-x86_64-unknown-linux-gnu https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_linux
chmod +x yt-dlp-x86_64-unknown-linux-gnu

curl -L -o yt-dlp-macos-temp https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_macos
chmod +x yt-dlp-macos-temp

cp yt-dlp-macos-temp yt-dlp-x86_64-apple-darwin
mv yt-dlp-macos-temp yt-dlp-aarch64-apple-darwin

echo "Done"