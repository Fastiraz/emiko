<div align="center">
  <h1><code>emiko</code></h1>
  <p>Never Google a command again.</p>
</div>

![](emiko.gif)

---

## What it do?

Emiko is an assistant that can execute commands on your shell.
It's written in Rust and works with [Ollama](https://ollama.com/).
The goal is to execute commands from natural language.
For example: `List me the content of ~/pwn folder.`.

## Installation

### Install Ollama

Emiko works with Ollama.
You will need to install Ollama and at least one model to use Emiko.
You can install Ollama [here](https://ollama.com/download).

After a few tests, we recommend the `qwen2.5-coder` model.
You can install it by running:

```bash
ollama pull qwen2.5-coder
```

If your machine is powerful enough, we recommend using the `qwen2.5-coder:14b` or `qwen2.5-coder:32b` model.

### Install Emiko

```bash
cargo install --git https://github.com/fastiraz/emiko.git
```

> [!IMPORTANT]
> If you have the following message after running the `cargo install` command, do not forget to add your cargo bin folder to your `PATH` to be able to run the installed binary.
>
> ```bash
> warning: be sure to add `~/.cargo/bin` to your PATH to be able to run the installed binaries
> ```
>
> Add the followinf line to your `.<shell>rc` file.
> ```bash
> export PATH="$HOME/.cargo/bin:$PATH"
> ```

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

## Roadmap

- [x] Support Ollama
- [x] Human CallBack Handler
- [x] Clipboard support
- [x] Support other providers
- [x] Implement debug/verbose mode
- [ ] Support images in prompt
