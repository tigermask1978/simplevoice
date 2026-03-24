use std::sync::Arc;
use tokio::sync::Mutex;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

// wk
use std::io::Write;
fn log_time(msg: &str) {
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("C:/temp/whisper_timing.log")
        .unwrap();
    writeln!(f, "{}: {}", chrono::Local::now(), msg).unwrap();
}
// 


pub async fn run(
    samples: &[f32],
    model_path: &str,
    language: &str,
    ctx_cache: Arc<Mutex<Option<(String, WhisperContext)>>>,
) -> anyhow::Result<String> {
    let mut cache = ctx_cache.lock().await;

    // wk
    // let t0 = std::time::Instant::now();

    if cache.as_ref().map(|(p, _)| p.as_str()) != Some(model_path) {
        let ctx = WhisperContext::new_with_params(model_path, WhisperContextParameters::default())
            .map_err(|e| anyhow::anyhow!("Failed to load model '{model_path}': {e}"))?;
        *cache = Some((model_path.to_string(), ctx));
    }

    //wk
    // tracing::info!("model load/cache: {:?}", t0.elapsed());
    // eprintln!("model load/cache: {:?}", t0.elapsed());
    // log_time(&format!("model load/cache: {:?}", t0.elapsed()));


    let ctx = &cache.as_ref().unwrap().1;

    //wk
    // let t1 = std::time::Instant::now();

    let mut state = ctx.create_state()?;

    //wk
    // tracing::info!("create_state: {:?}", t1.elapsed());
    // eprintln!("create_state: {:?}", t1.elapsed());
    // log_time(&format!("create_state: {:?}", t1.elapsed()));

    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    // wk
    let total = std::thread::available_parallelism()
        .map(|n| n.get() as i32)
        .unwrap_or(4);

    let n_threads = if total <= 4 {
        total  // 核心少就全用
    } else {
        (total / 2).min(8)  // 核心多就留一半给系统
    };
    params.set_n_threads(n_threads);
    // 加这一行确认线程数是否生效
    log_time(&format!("n_threads set to: {}", n_threads));
    // 
    params.set_language(Some(if language == "auto" { "auto" } else { language }));
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);

    //wk
    // let t2 = std::time::Instant::now();

    state.full(params, samples)?;

    //wk
    // tracing::info!("state.full inference: {:?}", t2.elapsed());
    // eprintln!("state.full inference: {:?}", t2.elapsed());
    // log_time(&format!("state.full inference: {:?}", t2.elapsed()));

    let text: String = state.as_iter().map(|seg| seg.to_string()).collect();

    //wk
    // tracing::info!("total transcribe: {:?}", t0.elapsed());
    // eprintln!("total transcribe: {:?}", t0.elapsed());
    // log_time(&format!("total transcribe: {:?}", t0.elapsed()));


    // wk
    // log_time(&format!("result text: {:?}", text));
    // 
    Ok(text.trim().to_string())
}
