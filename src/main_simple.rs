use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use hound::{SampleFormat, WavReader, WavSpec, WavWriter};

/// A program to cancel noise in WAV audio files
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input WAV file path
    #[arg(short, long)]
    input: PathBuf,

    /// Output WAV file path
    #[arg(short, long)]
    output: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("Processing audio file: {:?}", args.input);
    
    // Read the input WAV file
    let mut reader = WavReader::open(&args.input)
        .with_context(|| format!("Failed to open input file: {:?}", args.input))?;
    
    let spec = reader.spec();
    
    // Read audio samples
    let samples: Vec<f32> = if spec.sample_format == SampleFormat::Float {
        reader.samples::<f32>().map(|s| s.unwrap_or(0.0)).collect()
    } else {
        reader.samples::<i16>()
            .map(|s| s.unwrap_or(0) as f32 / 32768.0)
            .collect()
    };
    
    // Create output WAV file
    let output_spec = WavSpec {
        channels: spec.channels,
        sample_rate: spec.sample_rate,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };
    
    let mut writer = WavWriter::create(&args.output, output_spec)
        .with_context(|| format!("Failed to create output file: {:?}", args.output))?;
    
    // Write audio to output file (simple pass-through for now)
    for sample in samples {
        // Convert float to i16
        let sample_i16 = (sample.clamp(-1.0, 1.0) * 32767.0) as i16;
        writer.write_sample(sample_i16)?;
    }
    
    println!("Processing complete. Output saved to: {:?}", args.output);
    Ok(())
}