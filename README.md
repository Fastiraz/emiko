<center>
  <h1>emiko</h1>
  <p>Never Google a command again.</p>
</center>

---

## What it do?

Emiko is an assistant that can execute commands on your shell.
It's written in Rust and works with [Ollama](https://ollama.com/).
The goal is to execute commands from natural language.
For example: `List me the content of ~/pwn folder.`.

## Configuration

You can change the model by editing the `~/.config/emiko/emiko.json` file.
This file will be created automatically by starting emiko for the first time.
You can change the `model` attribute to use a different model or change the `url` attribute to use another ollama server.
