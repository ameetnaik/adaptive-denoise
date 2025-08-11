![](adaptive-denoise.png)

# Rust Noise Cancellation with DeepFilterNet

[![Rust Build](https://github.com/ameetnaik/adaptive-denoise/actions/workflows/rust.yml/badge.svg)](https://github.com/ameetnaik/adaptive-denoise/actions/workflows/rust.yml)

A Rust program that uses DeepFilterNet to cancel background noise and enhance speech in WAV audio files.

A Rust-based command-line interface for applying [DeepFilterNet](https://github.com/Rikorose/DeepFilterNet) noise suppression to `.wav` files.  
This tool allows you to easily denoise audio using DeepFilterNet's high-quality AI model from the comfort of your terminal.

## Features
- 🎙️ High-quality noise suppression for `.wav` files
- ⚡ Fast Rust-based implementation
- 🛠️ Simple CLI interface
- 📂 Works with pre-trained DeepFilterNet models
- 🛠️ Uses DeepFilterNet, a state-of-the-art deep learning model for speech enhancement and noise suppression
- 🎵 Handles both mono and multi-channel WAV files
- 🎼 Automatic resampling to match DeepFilterNet's required sample rate
- 🎼 Preserves original sample rate in the output file
- 📂 Command-line interface with input and output file parameters
- 🤿 Optional post-filter for enhanced noise reduction
- ❌ Comprehensive error handling with helpful messages

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/rust_nc.git
cd rust_nc

# Build the project
cargo build --release
```

## Usage

### Local Build
```bash
# Basic usage
cargo run --release -- -i input.wav -o output.wav

# Or using the compiled binary
./target/release/ad -i input.wav -o output.wav
```

### Docker
```bash
# Build the Docker image
docker build -t adaptive-denoise .

# Run with Docker (mount current directory)
docker run --rm -v $(pwd):/data adaptive-denoise -i /data/input.wav -o /data/output.wav
```

### Command-line Options

- `-i, --input <FILE>`: Input WAV file path (required)
- `-o, --output <FILE>`: Output WAV file path (required)
- `--pf`: Enable post-filter for better noise reduction
- `--pf-beta <FLOAT>`: Post-filter beta value (default: 0.02). Higher values result in stronger noise attenuation
- `-v, --verbose`: Increase logging verbosity (use multiple times for more detail)
- `-a, --atten-lim-db <FLOAT>`: Attenuation limit in dB (default: 100.0)
- `--min-db-thresh <FLOAT>`: Min dB local SNR threshold (default: -15.0)
- `--max-db-erb-thresh <FLOAT>`: Max dB local SNR threshold for ERB decoder (default: 35.0)
- `--max-db-df-thresh <FLOAT>`: Max dB local SNR threshold for DF decoder (default: 35.0)
- `-D, --compensate-delay`: Compensate delay of STFT and model lookahead

## Technical Details

- The program reads the input WAV file using the `hound` crate
- Audio is converted to mono if it has multiple channels
- DeepFilterNet works best with 48kHz sample rate audio
- The audio is processed through the DeepFilterNet model for noise cancellation and speech enhancement
- The enhanced audio is saved to the specified output file in 16-bit PCM format
- Progress is displayed during processing showing the amount of audio processed
- Supports both single-channel and multi-channel audio files
- Automatic resampling when input sample rate differs from model requirements

## Dependencies

- Local implementation of DeepFilterNet
- `hound`: For reading and writing WAV files
- `clap`: For command-line argument parsing
- `anyhow`: For error handling
- `ndarray`: For multi-dimensional array operations
- `tract-core`, `tract-onnx`, `tract-pulse`: For ONNX model inference
- `rubato`: For audio resampling
- `pbr`: For progress bar display
- `rust-embed`: For embedding model files

## Notes

- The DeepFilterNet model is embedded in the binary, so no internet connection is required
- For best results, use input audio with a 48kHz sample rate
- The program automatically converts multi-channel audio to mono for processing
- Processing time depends on audio length and system performance
- Use `-v` flag for verbose output to monitor processing details
- The program creates output directories automatically if they don't exist

---

## How This Differs from the Original DeepFilterNet

The original [DeepFilterNet](https://github.com/Rikorose/DeepFilterNet) project is written in Python and designed for real-time speech enhancement with deep learning.  
This Rust implementation:
- Provides a **CLI tool** for offline noise suppression on `.wav` files
- Focuses on **batch processing** rather than live audio streaming
- Uses Rust for speed and safety
- Integrates only the model inference portion of DeepFilterNet, not the training pipeline

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

> **Note:** The `LICENSE` file in this repository contains:
> - The MIT License for this Rust CLI implementation (© 2025 Ameet Naik)
> - The MIT License for DeepFilterNet (© 2021 Reinhard Köhler)

---

## Acknowledgements

This project builds upon **[DeepFilterNet](https://github.com/Rikorose/DeepFilterNet)** from Reinhard Köhler Copyright (c) 2021 
Licensed under the [MIT License](https://github.com/Rikorose/DeepFilterNet/blob/main/LICENSE).

DeepFilterNet is an excellent real-time speech enhancement project that uses deep learning to remove background noise from speech audio.  
This repository provides a Rust CLI wrapper to run inference on `.wav` files using DeepFilterNet.
