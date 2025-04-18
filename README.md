# 🎵 Lyrics Scraper in Rust

A fast, reliable, and extensible Rust-based lyrics fetcher. It first tries to retrieve lyrics from the lightweight [Lyrics.ovh](https://lyricsovh.docs.apiary.io/#reference/0/lyrics-of-a-song/search) API. If the lyrics are not found there, it gracefully falls back to scraping them from [letras.com](https://www.letras.com).

📝 After fetching the lyrics, the tool writes them to a `.lrc` file placed next to the input `.mp3` file.

---

## ✨ Features

- ⚡ **Fast lyrics fetching** via Lyrics.ovh
- 🔁 **Automatic fallback** to letras.com web scraping
- 📄 **Writes lyrics** to a `.lrc` file beside the `.mp3`
- 🧠 **Smart slug generation** for building letras.com URLs
- 🌐 **Blocking HTTP requests** — ideal for synchronous tools or CLIs
- 🔍 **Precise HTML parsing** using [`scraper`](https://docs.rs/scraper)
- 🧩 **Trait-based architecture** for multiple backend implementations
- 🧪 **Real integration tests** (no mocks!)
- 📦 Usable as a binary or library
- 🐳 Docker image support for easy builds

---

## 📦 Installation

```bash
git clone https://github.com/albertjimenez/GetLyrics.git
cd GetLyrics
cargo build --release
```

---

## 🚀 Usage (CLI)

Once compiled, you can run the binary with an `.mp3` file as an argument:

```bash
./getlyrics "/absolute/path/to/song.mp3"
```

It will extract the metadata (artist/title), attempt to fetch the lyrics, and save them into a `.lrc` file in the same directory:

```bash
/absolute/path/to/song.lrc
```

---

### 🔁 Batch Usage

To recursively search for `.mp3` files and fetch lyrics for each:

```bash
find . -type f -name '*.mp3' -exec ./getlyrics "$(pwd)/{}" \;
```

To recursively search for `.flac` files and fetch lyrics for each:

```bash
find . -type f -name '*.flac' -exec ./getlyrics "$(pwd)/{}" \;
```

---

## 🐳 Docker Support

You can build the Docker image locally:

```bash
docker build -t beruto/getlyrics:1 .
```

Once built, use it like this:

```bash
docker run --rm -v "$(pwd)":/music beruto/getlyrics:1 /music/song.mp3
```

> Replace `/music/song.mp3` with the absolute path inside the mounted volume.

---

## 🧪 Tests

Run real-world integration tests:

```bash
cargo test -- --nocapture
```

> These tests make actual HTTP requests to Lyrics.ovh and letras.com.

---


## 🛠 Technologies Used

- [Rust](https://www.rust-lang.org/)
- [reqwest](https://docs.rs/reqwest/)
- [scraper](https://docs.rs/scraper/)
- [Lyrics.ovh API](https://lyricsovh.docs.apiary.io)
- [letras.com](https://www.letras.com)

---

## 📜 License

MIT License — see [`LICENSE`](./LICENSE) for details.

---

## 🤝 Contributing

Issues and PRs are welcome! If you have an idea for another fallback provider (e.g., Genius, Musixmatch), open a discussion or send a PR.
