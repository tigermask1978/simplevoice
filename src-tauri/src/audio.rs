use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rubato::{FftFixedIn, Resampler};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter};

struct SendStream(cpal::Stream);
// SAFETY: We ensure the stream is only accessed while holding the Recorder mutex.
unsafe impl Send for SendStream {}

pub struct Recorder {
    samples: Arc<Mutex<Vec<f32>>>,
    stream: Option<SendStream>,
    /// actual device config used for recording
    device_config: Option<(u32, u16)>, // (sample_rate, channels)
}

impl Recorder {
    pub fn new() -> Self {
        Self { samples: Arc::new(Mutex::new(Vec::new())), stream: None, device_config: None }
    }

    pub fn start(&mut self) -> anyhow::Result<()> {
        let host = cpal::default_host();
        let device = host.default_input_device().ok_or_else(|| anyhow::anyhow!("No input device"))?;
        let supported = device.default_input_config()?;
        let sr = supported.sample_rate().0;
        let ch = supported.channels();
        tracing::info!("Recording at {}Hz, {} channel(s)", sr, ch);

        let config = cpal::StreamConfig {
            channels: ch,
            sample_rate: supported.sample_rate(),
            buffer_size: cpal::BufferSize::Default,
        };

        self.samples.lock().unwrap().clear();
        self.device_config = Some((sr, ch));
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

    /// Stop recording and return 16kHz mono f32 samples.
    pub fn stop(&mut self) -> Vec<f32> {
        self.stream.take();
        let raw = std::mem::take(&mut *self.samples.lock().unwrap());
        let (sr, ch) = self.device_config.take().unwrap_or((16000, 1));
        to_16k_mono(raw, sr, ch as usize)
    }
}

/// Convert interleaved multi-channel audio at `src_sr` Hz to 16kHz mono f32.
fn to_16k_mono(raw: Vec<f32>, src_sr: u32, channels: usize) -> Vec<f32> {
    // 1. Mix down to mono
    let mono: Vec<f32> = if channels == 1 {
        raw
    } else {
        raw.chunks_exact(channels)
            .map(|frame| frame.iter().sum::<f32>() / channels as f32)
            .collect()
    };

    const TARGET_SR: u32 = 16000;
    if src_sr == TARGET_SR {
        return mono;
    }

    // 2. Resample to 16kHz using rubato FftFixedIn
    let chunk = 1024usize;
    let mut resampler =
        FftFixedIn::<f32>::new(src_sr as usize, TARGET_SR as usize, chunk, 2, 1)
            .expect("resampler init");

    let mut out = Vec::with_capacity(mono.len() * TARGET_SR as usize / src_sr as usize + chunk);
    let mut pos = 0;
    while pos + chunk <= mono.len() {
        let input = vec![mono[pos..pos + chunk].to_vec()];
        let result = resampler.process(&input, None).expect("resample");
        out.extend_from_slice(&result[0]);
        pos += chunk;
    }
    // tail
    if pos < mono.len() {
        let mut tail = mono[pos..].to_vec();
        tail.resize(chunk, 0.0);
        let input = vec![tail];
        let result = resampler.process(&input, None).expect("resample tail");
        let expected_tail = (mono.len() - pos) * TARGET_SR as usize / src_sr as usize;
        out.extend_from_slice(&result[0][..expected_tail.min(result[0].len())]);
    }

    tracing::info!("Resampled {}Hz→16kHz: {} → {} samples", src_sr, mono.len(), out.len());
    out
}

#[tauri::command]
pub async fn start_recording(
    state: tauri::State<'_, crate::AppState>,
    app: AppHandle,
) -> Result<(), String> {
    state.recorder.lock().await.start().map_err(|e| e.to_string())?;
    crate::tray::set_recording(&app, true);
    app.emit("recording-state", "recording").ok();
    Ok(())
}

#[tauri::command]
pub async fn stop_recording(
    state: tauri::State<'_, crate::AppState>,
    app: AppHandle,
) -> Result<(), String> {
    let samples = state.recorder.lock().await.stop();
    crate::tray::set_recording(&app, false);
    app.emit("recording-state", "transcribing").ok();

    let config = state.config.lock().await.clone();
    let ctx_cache = state.whisper_ctx.clone();
    let app2 = app.clone();

    tauri::async_runtime::spawn(async move {
        match crate::transcribe::run(&samples, &config.model_path, &config.language, ctx_cache).await {
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
