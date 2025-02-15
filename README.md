<div align="center">
  <h1>emiko</h1>
  <p>Never Google a command again.</p>
</div>

---

## What it do?

Emiko is an assistant that can execute commands on your shell.
It's written in Rust and works with [Ollama](https://ollama.com/).
The goal is to execute commands from natural language.
For example: `List me the content of ~/pwn folder.`.

## Installation

### Build from source

```bash
git clone --depth 1 https://github.com/Fastiraz/emiko.git
cd emiko
cargo build --release
```

Run from build directory by executing it directly from the target directory:

```bash
./target/release/emiko
```

If you want to install the binary on your system, you can move the binary in your `/bin`, `/usr/bin`, `~/.local/bin` or `/usr/local/bin`.

```bash
mv ./target/release/emiko /usr/local/bin
```

### Verify installation

```bash
emiko --version
```

## Configuration

You can change the model by editing the `~/.config/emiko/emiko.json` file.
This file will be created automatically by starting emiko for the first time.
You can change the `model` attribute to use a different model or change the `url` attribute to use another ollama server.
