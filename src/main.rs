use anyhow::{bail, Context, Result};
use clap::Parser;
use std::{fs::File, path::Path, path::PathBuf};

use rubato::{FftFixedIn, Resampler};

use symphonia::core::{
    audio::SampleBuffer,
    codecs::DecoderOptions,
    formats::FormatOptions,
    io::MediaSourceStream,
    meta::MetadataOptions,
    probe::Hint,
};

use whisper_rs::{
    FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters,
};

#[derive(Parser, Debug)]
struct Args {
    /// Modèle Whisper ggml (ex: ggml-base.bin)
    #[arg(long)]
    model: PathBuf,

    /// Fichier audio .m4a
    #[arg(long)]
    m4a: PathBuf,

    /// Langue (ex: fr). Vide => auto
    #[arg(long, default_value = "fr")]
    lang: String,

    /// Threads whisper (0 => défaut)
    #[arg(long, default_value_t = 0)]
    threads: i32,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // 1) Decode m4a -> mono f32 + sr_in
    let (mono, sr_in) = decode_m4a_to_f32_mono(&args.m4a).context("decode m4a")?;

    // 2) Resample -> 16k
    let samples_16k = resample_to_16k_mono(&mono, sr_in).context("resample to 16k")?;

    // 3) Whisper context (API v0.15.x)
    let mut ctx_params = WhisperContextParameters::default();
    // ctx_params.use_gpu = false; // optionnel selon plateforme

    let model_path = args.model.to_string_lossy().to_string();
    let ctx = WhisperContext::new_with_params(&model_path, ctx_params).context("load model")?;
    let mut state = ctx.create_state().context("create_state")?;

    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });

    if args.threads > 0 {
        params.set_n_threads(args.threads);
    }

    if args.lang.trim().is_empty() {
        params.set_language(None);
    } else {
        params.set_language(Some(args.lang.trim()));
    }

    params.set_translate(false);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_special(false);

    state.full(params, &samples_16k).context("whisper full")?;

    let n = state.full_n_segments() as usize;
    for i in 0..n {
        if let Some(seg) = state.get_segment(i as i32) {
            println!("{}", seg);
        }
    }

    Ok(())
}

/// Décode un .m4a (AAC) en mono f32.
/// Retourne (samples_mono_f32, sample_rate_hz)
fn decode_m4a_to_f32_mono(path: &Path) -> Result<(Vec<f32>, u32)> {
    let file = File::open(path).with_context(|| format!("open {:?}", path))?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let mut hint = Hint::new();
    hint.with_extension("m4a");

    let probed = symphonia::default::get_probe()
        .format(
            &hint,
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .context("probe format")?;

    let mut format = probed.format;

    // ⚠️ Fix du borrow: on copie ce dont on a besoin (id, sr, channels) puis on relâche track
    let (track_id, sr) = {
        let track = format.default_track().context("no default audio track")?;
        let sr = track.codec_params.sample_rate.context("unknown sample rate")?;
        (track.id, sr)
    };

    // Reprend la piste via track_id pour instancier le decoder
    let track = format
        .tracks()
        .iter()
        .find(|t| t.id == track_id)
        .context("track not found")?;

    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &DecoderOptions::default())
        .context("make decoder")?;

    let mut out: Vec<f32> = Vec::new();

    loop {
        let packet = match format.next_packet() {
            Ok(p) => p,
            Err(err) => {
                if matches!(err, symphonia::core::errors::Error::IoError(_)) {
                    break;
                }
                return Err(err).context("next_packet").map_err(Into::into);
            }
        };

        if packet.track_id() != track_id {
            continue;
        }

        let decoded = decoder.decode(&packet).context("decode")?;

        // Convertit vers f32 interleaved, quel que soit le format interne
        // Déduire le nombre de canaux depuis la spécification du buffer décodé
        let channels = decoded.spec().channels.count();
        let mut sbuf = SampleBuffer::<f32>::new(decoded.capacity() as u64, *decoded.spec());
        sbuf.copy_interleaved_ref(decoded);
        let samples = sbuf.samples();

        if channels == 1 {
            out.extend_from_slice(samples);
        } else {
            // downmix simple : moyenne des canaux
            for frame in samples.chunks(channels) {
                let mut sum = 0.0f32;
                for &s in frame {
                    sum += s;
                }
                out.push(sum / channels as f32);
            }
        }
    }

    if out.is_empty() {
        bail!("audio vide ou non décodable: {:?}", path);
    }

    Ok((out, sr))
}

/// Resample mono f32 vers 16 kHz en utilisant un resampler FFT (API rubato 0.16.x)
fn resample_to_16k_mono(input: &[f32], sr_in: u32) -> Result<Vec<f32>> {
    if sr_in == 16_000 {
        return Ok(input.to_vec());
    }

    let sr_out = 16_000u32;

    // Paramètres FFT : compromis simple pour de la voix
    let chunk_size = 2048usize;     // taille d'entrée par bloc
    let sub_chunks = 2usize;        // latence/qualité
    let channels = 1usize;

    let mut resampler = FftFixedIn::<f32>::new(
        sr_in as usize,
        sr_out as usize,
        chunk_size,
        sub_chunks,
        channels,
    )
    .context("resampler init")?;

    let mut out = Vec::<f32>::new();
    let mut idx = 0;

    while idx < input.len() {
        let end = (idx + chunk_size).min(input.len());
        let mut chunk = input[idx..end].to_vec();
        if chunk.len() < chunk_size {
            chunk.resize(chunk_size, 0.0);
        }

        let y = resampler.process(&[chunk], None).context("resampler process")?;
        out.extend_from_slice(&y[0]);
        idx = end;
    }

    Ok(out)
}
