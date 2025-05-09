# 🎵 GetLyrics (Rust-based Lyrics Fetcher)

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/albertjimenez/GetLyrics)


A fast, reliable, and extensible Rust-based lyrics fetcher.

✨ **New in latest version:**
- ❌ **No more web scraping** – cleaner, more stable code.
- 📉 **Smaller binary size** due to removal of HTML parser and scraping logic.
- 🎤 **New `--karaoke` mode** to fetch synced lyrics when available.

📝 After fetching the lyrics, the tool writes them to a `.lrc` file placed next to the input audio file (`.mp3`, `.flac`, etc.).

---

## ✨ Features

- ⚡ **Fast lyrics fetching** via [LRCLib](https://lrclib.net) API
- 📄 **Writes lyrics** to a `.lrc` file beside the input song
- 🔁 **Fallback handling** for tracks with slight duration mismatches
- 🎤 **Karaoke mode**: get synced `.lrc` lyrics when available
- 🌐 **Blocking HTTP requests** — ideal for CLI and scripts
- 🧩 **Trait-based architecture** for future backend extensions
- 🧪 **Real integration tests**
- 📦 Usable as a binary or library
- 🐳 Docker image support for easy builds and usage

---

## 📦 Installation

```bash
git clone https://github.com/albertjimenez/GetLyrics.git
cd GetLyrics
cargo build --release
```

---

## 🚀 Usage (CLI)

Once compiled, run it with an `.mp3` or `.flac` file:

```bash
./getlyrics "/absolute/path/to/song.mp3"
```

Use the `-k` or `--karaoke` flag to request synced lyrics (if available):

```bash
./getlyrics --karaoke "/absolute/path/to/song.mp3"
```

This will extract metadata, fetch lyrics from LRCLib, and save the result to:

```bash
/absolute/path/to/song.lrc
```

---

### 🔁 Batch Usage

To recursively fetch lyrics for all `.mp3` or `.flac` files:

```bash
find . -type f -name '*.mp3' -exec ./getlyrics "$(pwd)/{}" \;
find . -type f -name '*.flac' -exec ./getlyrics "$(pwd)/{}" \;
```

---

## 🐳 Docker Support

The latest version is already available on Docker Hub:

```bash
docker pull beruto/getlyrics:2
```

Use it like this:

```bash
docker run --rm -v "$(pwd)":/music beruto/getlyrics:2 /music/song.mp3
```

To enable synced lyrics (karaoke mode):

```bash
docker run --rm -v "$(pwd)":/music beruto/getlyrics:2 --karaoke /music/song.mp3
```

> Replace `/music/song.mp3` with the correct path inside the mounted volume.

---

## 🧪 Tests

Run real-world integration tests:

```bash
cargo test -- --nocapture
```

> These tests make actual HTTP requests to LRCLib.

---

## 🛠 Technologies Used

- [Rust](https://www.rust-lang.org/)
- [reqwest](https://docs.rs/reqwest/)
- [LRCLib](https://lrclib.net) – open lyrics API

---

## 📜 License

MIT License — see [`LICENSE`](./LICENSE) for details.

---

## 🤝 Contributing

PRs and issues are welcome! Have an idea for a new lyrics provider or format? Open an issue or fork and contribute.
