import type { SearchResultItem } from "$lib/bindings.js";
import { commands } from "$lib/bindings.js";

class PlayerState {
  currentTrack = $state<SearchResultItem | null>(null);
  isPlaying = $state(false);
  progress = $state(0);
  duration = $state(0);
  volume = $state(1);
  isLoading = $state(false);

  private progressInterval: ReturnType<typeof setInterval> | null = null;

  async play(item: SearchResultItem) {
    if (this.currentTrack?.id === item.id) {
      this.toggle();
      return;
    }

    this.currentTrack = item;
    this.isPlaying = false;
    this.progress = 0;
    this.duration = 0;
    this.isLoading = true;

    try {
      // Parse duration from string like "4:05" to seconds
      if (item.duration) {
        const parts = item.duration.split(":").map(Number);
        if (parts.length === 2) {
          this.duration = parts[0] * 60 + parts[1];
        } else if (parts.length === 3) {
          this.duration = parts[0] * 3600 + parts[1] * 60 + parts[2];
        }
      } else {
        this.duration = 0; // Unknown duration
      }

      const res = await commands.playAudio(item.id);
      if (res.status === "ok") {
        this.isPlaying = true;
        this.startPolling();
      } else {
        console.error("Failed to play audio:", res.error);
        this.isPlaying = false;
      }
    } catch (e) {
      console.error(e);
      this.isPlaying = false;
    } finally {
      this.isLoading = false;
    }
  }

  private startPolling() {
    this.stopPolling();
    this.progressInterval = setInterval(async () => {
      if (this.isPlaying && this.duration > 0) {
        const currentSecs = await commands.getAudioProgress();
        if (currentSecs !== null) {
          this.progress = (currentSecs / this.duration) * 100;
          if (currentSecs >= this.duration) {
            this.isPlaying = false;
            this.stopPolling();
            this.progress = 0;
          }
        }
      }
    }, 1000);
  }

  private stopPolling() {
    if (this.progressInterval) {
      clearInterval(this.progressInterval);
      this.progressInterval = null;
    }
  }

  async toggle() {
    if (this.isPlaying) {
      await commands.pauseAudio();
      this.isPlaying = false;
      this.stopPolling();
    } else {
      await commands.resumeAudio();
      this.isPlaying = true;
      this.startPolling();
    }
  }

  async setVolume(vol: number) {
    this.volume = vol;
    await commands.setVolume(vol);
  }

  async seek(percentage: number) {
    if (this.duration) {
      const position = (percentage / 100) * this.duration;
      this.progress = percentage;
      await commands.seekAudio(position);
    }
  }
}

export const player = new PlayerState();
