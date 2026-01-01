use anyhow::Result;
use rodio::Decoder;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn get_cache_path(url: &str) -> PathBuf {
    let mut hasher = Sha256::new();
    hasher.update(url.as_bytes());
    let hash = format!("{:x}", hasher.finalize());

    let cache_dir = dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("auria_audio_cache");

    fs::create_dir_all(&cache_dir).ok();
    cache_dir.join(format!("{}.mp3", &hash[..16]))
}

pub struct AudioPlayer {
    _stream_handle: rodio::OutputStream,
    sink: rodio::Sink,
}

impl Default for AudioPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioPlayer {
    pub fn new() -> Self {
        let stream_handle =
            rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
        let sink = rodio::Sink::connect_new(&stream_handle.mixer());

        Self {
            _stream_handle: stream_handle,
            sink,
        }
    }

    pub async fn play_audio(&mut self, url: &str) -> Result<()> {
        // clear if there a song playing
        if !self.sink.empty() {
            self.sink.clear();
        }

        let cache_path = get_cache_path(url);

        let audio_data = if cache_path.exists() {
            fs::read(&cache_path)?
        } else {
            let mut ytdlp = Command::new("yt-dlp")
                .args(["-f", "bestaudio", "-o", "-", url])
                .stdout(Stdio::piped())
                .spawn()?;

            let ffmpeg = Command::new("ffmpeg")
                .args([
                    "-i",
                    "pipe:0",
                    "-af",
                    "loudnorm=I=-16:TP=-1.5:LRA=11,volume=0.5",
                    "-f",
                    "mp3",
                    "-b:a",
                    "128k",
                    "-q:a",
                    "2",
                    "pipe:1",
                ])
                .stdin(ytdlp.stdout.take().unwrap())
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()?;

            let output = ffmpeg.wait_with_output()?;
            let data = output.stdout;

            // Guardar en caché
            fs::write(&cache_path, &data).ok();

            data
        };

        let cursor = std::io::Cursor::new(audio_data);
        let source = Decoder::new(cursor)?;

        self.sink.append(source);
        self.sink.play();

        Ok(())
    }

    pub fn pause(&mut self) -> Result<()> {
        self.sink.pause();

        Ok(())
    }

    pub fn resume(&mut self) {
        self.sink.play();
    }
}
