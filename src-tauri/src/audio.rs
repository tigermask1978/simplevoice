use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter};

// cpal::Stream is !Send on Windows due to COM threading, but we only
// access it from a single async task at a time via the AppState Mutex.
struct SendStream(cpal::Stream);
// SAFETY: We ensure the stream is only accessed while holding the Recorder mutex.
unsafe impl Send for SendStream {}

pub struct Recorder {
    samples: Arc<Mutex<Vec<f32>>>,
    stream: Option<SendStream>,
}

impl Recorder {
    pub fn new() -> Self {
        Self { samples: Arc::new(Mutex::new(Vec::new())), stream: None }
    }

    pub fn start(&mut self) -> anyhow::Result<()> {
        let host = cpal::default_host();
        let device = host.default_input_device().ok_or_else(|| anyhow::anyhow!("No input device"))?;
        let config = cpal::StreamConfig {
            channels: 1,
            sample_rate: cpal::SampleRate(16000),
            buffer_size: cpal::BufferSize::Default,
        };

        self.samples.lock().unwrap().clear();
        let samples = Arc::clone(&self.samples);

        let stream = device.build_input_stream(
            &config,
            move |data: &[f32], _| {
                samples.lock().unwrap().extend_from_slice(data);
            },
            |e| tracing::error!("Audio stream error: {e}"),
            None,
        )?;
        stream.play()?;
        self.stream = Some(SendStream(stream));
        Ok(())
    }

    pub fn stop(&mut self) -> Vec<f32> {
        self.stream.take(); // drops stream, stops recording
        std::mem::take(&mut *self.samples.lock().unwrap())
    }
}

#[tauri::command]
pub async fn start_recording(
    state: tauri::State<'_, crate::AppState>,
    app: AppHandle,
) -> Result<(), String> {
    state.recorder.lock().await.start().map_err(|e| e.to_string())?;
    app.emit("recording-state", "recording").ok();
    Ok(())
}

#[tauri::command]
pub async fn stop_recording(
    state: tauri::State<'_, crate::AppState>,
    app: AppHandle,
) -> Result<(), String> {
    let samples = state.recorder.lock().await.stop();
    app.emit("recording-state", "transcribing").ok();

    let config = state.config.lock().await.clone();
    let app2 = app.clone();

    tokio::spawn(async move {
        match crate::transcribe::run(&samples, &config.model_path, &config.language) {
            Ok(text) => {
                app2.emit("recording-state", "idle").ok();
                if !text.is_empty() {
                    app2.emit("transcription-result", &text).ok();
                    if let Err(e) = crate::inject::type_text(&text) {
                        app2.emit("transcription-error", e.to_string()).ok();
                    }
                }
            }
            Err(e) => {
                app2.emit("recording-state", "idle").ok();
                app2.emit("transcription-error", e.to_string()).ok();
            }
        }
    });

    Ok(())
}
