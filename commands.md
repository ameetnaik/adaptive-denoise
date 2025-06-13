




ffmpeg -i https://pdl-iphone-cnbc-com.akamaized.net/VCPS/Y2024/M08D02/7000347658/1722633863-35643859315-hd_MBR_4500.mp4 -ar 48000 input.wav


RUST_BACKTRACE=full cargo run -- -v -m ./models/DeepFilterNet3_ll_onnx.tar.gz input.wav


RUST_BACKTRACE=full cargo run -- -v input.wav