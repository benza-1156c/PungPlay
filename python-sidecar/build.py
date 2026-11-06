import subprocess
import shutil
import sys
import platform
from pathlib import Path

def get_target_triple() -> str:
    system = platform.system()
    machine = platform.machine().lower()
    
    if system == "Windows":
        return "x86_64-pc-windows-msvc"
    elif system == "Darwin":
        return "aarch64-apple-darwin" if machine == "arm64" else "x86_64-apple-darwin"
    elif system == "Linux":
        if machine in ("aarch64", "arm64"):
            return "aarch64-unknown-linux-gnu"
        return "x86_64-unknown-linux-gnu"
    else:
        raise RuntimeError(f"Unsupported OS: {system}")

def main():
    here = Path(__file__).parent
    target_triple = get_target_triple()
    ext = ".exe" if platform.system() == "Windows" else ""
    
    if not (here / "vendor").exists():
        print(" vendor/ 'uv pip install -r requirements.txt --target ./vendor' ก่อน")
        sys.exit(1)
    
    print(f"🔨 Building for {target_triple}...")
    subprocess.run([
        sys.executable, "-m", "PyInstaller",
        "--onefile",
        "--name", "yt-sidecar",
        "--paths", str(here / "vendor"),
        "--clean",
        "--noconfirm",
        str(here / "main.py"),
    ], check=True)
    
    src = here / "dist" / f"yt-sidecar{ext}"
    dst_dir = here.parent / "src-tauri" / "binaries"
    dst_dir.mkdir(parents=True, exist_ok=True)
    dst = dst_dir / f"yt-sidecar-{target_triple}{ext}"
    
    shutil.copy2(src, dst)
    print(f"Copied to: {dst}")

if __name__ == "__main__":
    main()