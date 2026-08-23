use rodio::{OutputStream, OutputStreamHandle, Sink};
use std::sync::Mutex;

pub struct AudioState {
    pub sink: Mutex<Option<Sink>>,
    pub stream_handle: OutputStreamHandle,
    pub current_volume: Mutex<f32>,
}

impl AudioState {
    pub fn new() -> Result<Self, String> {
        let (stream, stream_handle) =
            OutputStream::try_default().map_err(|e| format!("ไม่สามารถเริ่มระบบเสียงได้: {}", e))?;

        // Leak the stream so it stays alive for the entire app lifetime.
        // This is necessary because OutputStream is not Send + Sync.
        Box::leak(Box::new(stream));

        Ok(Self {
            sink: Mutex::new(None),
            stream_handle,
            current_volume: Mutex::new(1.0),
        })
    }

    pub fn play_file(&self, path: &std::path::Path) -> Result<(), String> {
        let file = std::fs::File::open(path).map_err(|e| format!("ไม่สามารถเปิดไฟล์เสียงได้: {}", e))?;
        let reader = std::io::BufReader::new(file);
        let source = rodio::Decoder::new(reader)
            .map_err(|e| format!("ไม่สามารถถอดรหัสเสียงได้ (Codec): {}", e))?;

        // Recreate sink to clear old audio
        let new_sink = Sink::try_new(&self.stream_handle)
            .map_err(|e| format!("ไม่สามารถสร้างช่องเสียงได้: {}", e))?;

        let vol = *self.current_volume.lock().unwrap();
        new_sink.set_volume(vol);
        new_sink.append(source);
        new_sink.play();

        let mut sink_guard = self.sink.lock().unwrap();
        *sink_guard = Some(new_sink);

        Ok(())
    }

    pub fn pause(&self) {
        if let Some(sink) = self.sink.lock().unwrap().as_ref() {
            sink.pause();
        }
    }

    pub fn resume(&self) {
        if let Some(sink) = self.sink.lock().unwrap().as_ref() {
            sink.play();
        }
    }

    pub fn set_volume(&self, volume: f32) {
        *self.current_volume.lock().unwrap() = volume;
        if let Some(sink) = self.sink.lock().unwrap().as_ref() {
            sink.set_volume(volume);
        }
    }

    pub fn get_position(&self) -> f32 {
        if let Some(sink) = self.sink.lock().unwrap().as_ref() {
            sink.get_pos().as_secs_f32()
        } else {
            0.0
        }
    }

    pub fn seek(&self, position_secs: f32) {
        if let Some(sink) = self.sink.lock().unwrap().as_ref() {
            let duration = std::time::Duration::from_secs_f32(position_secs);
            let _ = sink.try_seek(duration);
        }
    }
}
