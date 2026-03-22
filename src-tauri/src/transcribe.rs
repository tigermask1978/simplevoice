use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

pub fn run(samples: &[f32], model_path: &str, language: &str) -> anyhow::Result<String> {
    let ctx = WhisperContext::new_with_params(model_path, WhisperContextParameters::default())
        .map_err(|e| anyhow::anyhow!("Failed to load model '{model_path}': {e}"))?;

    let mut state = ctx.create_state()?;

    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    params.set_language(Some(if language == "auto" { "auto" } else { language }));
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);

    state.full(params, samples)?;

    let text: String = state.as_iter().map(|seg| seg.to_string()).collect();
    Ok(text.trim().to_string())
}
