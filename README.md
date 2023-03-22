uses https://llvm-mos.org and https://github.com/mrk-its/rust-mos read more https://llvm-mos.org/wiki/Rust 

public domain font https://opengameart.org/content/intrepid-monochrome-8-bit-font

building;

```bash
docker pull mrkits/rust-mos
docker run -it --name rustmos --entrypoint bash -v ${HOME}/Documents:/hostfiles mrkits/rust-mos
docker container exec -it rustmos /bin/bash
cargo build --target mos-nes-cnrom-none
```
