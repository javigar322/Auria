# Youtube Music Desktop App

Una aplicación de escritorio para reproducir música desde YouTube, construida en **Rust** usando **eframe/egui** y **rodio**. Permite reproducir, pausar y reanudar canciones de manera rápida y eficiente, con almacenamiento en caché local para mejorar el rendimiento.

---

## 🚀 Características

- Reproducción de audio desde URLs de YouTube.
- Pausar y reanudar canciones al instante.
- Cache local de audio para evitar descargas repetidas.
- UI moderna con **egui** y tema **Catppuccin Macchiato**.
- Arquitectura async con **Tokio** para no bloquear la interfaz.
- Multi-tasking seguro usando canales (`mpsc`) para manejar comandos de audio.

---

## 🛠 Tecnologías utilizadas

- **Rust** – Lenguaje principal
- **eframe / egui** – Framework GUI
- **rodio** – Reproducción de audio
- **Tokio** – Runtime async
- **yt-dlp** – Descarga de audio de YouTube
- **ffmpeg** – Procesamiento y conversión de audio

---

## 💾 Instalación

1. Clona el repositorio:

```bash
git clone https://github.com/javigar322/Auria
```

2. build del repositorio:

```bash
cargo build --realease
```
