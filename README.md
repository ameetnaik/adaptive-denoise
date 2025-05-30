# Rust Noise Cancellation with DeepFilterNet

A Rust program that uses DeepFilterNet to cancel background noise and enhance speech in WAV audio files.

## Features

- Uses DeepFilterNet, a state-of-the-art deep learning model for speech enhancement and noise suppression
- Handles both mono and multi-channel WAV files
- Automatic resampling to match DeepFilterNet's required sample rate
- Preserves original sample rate in the output file
- Command-line interface with input and output file parameters
- Optional post-filter for enhanced noise reduction
- Comprehensive error handling with helpful messages

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/rust_nc.git
cd rust_nc

# Build the project
cargo build --release
```

## Usage

```bash
# Basic usage
cargo run --release -- -i input.wav -o output.wav

# Or using the compiled binary
./target/release/rust_nc -i input.wav -o output.wav
```

### Command-line Options

- `-i, --input <FILE>`: Input WAV file path (required)
- `-o, --output <FILE>`: Output WAV file path (required)
- `--pf`: Enable post-filter for better noise reduction
- `--pf-beta <FLOAT>`: Post-filter beta value (default: 0.02). Higher values result in stronger noise attenuation

## Technical Details

- The program reads the input WAV file using the `hound` crate
- Audio is converted to mono if it has multiple channels
- DeepFilterNet works best with 48kHz sample rate audio
- The audio is processed through the DeepFilterNet model for noise cancellation and speech enhancement
- The enhanced audio is saved to the specified output file in 16-bit PCM format

## Dependencies

- Local implementation of DeepFilterNet
- `hound`: For reading and writing WAV files
- `clap`: For command-line argument parsing
- `anyhow`: For error handling

## Notes

- The first time you run the program, it will download the DeepFilterNet model, which might take a moment depending on your internet connection
- For best results, use input audio with a 48kHz sample rate
- The program automatically converts audio to mono as DeepFilterNet processes single-channel audio