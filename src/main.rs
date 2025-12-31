use egui_async::EguiAsyncPlugin;
use tokio::sync::mpsc;

use crate::player::player::AudioPlayer;

mod player;

#[derive(Clone)]
enum PlayerCommand {
    Play(String),
    Pause,
    Resume,
}

struct Auria {
    //audio_bind: Bind<(), String>,
    tx: mpsc::UnboundedSender<PlayerCommand>,
}

impl Auria {
    fn new(cc: &eframe::CreationContext, tx: mpsc::UnboundedSender<PlayerCommand>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let mut fonts = egui::FontDefinitions::default();
        egui_remixicon::add_to_fonts(&mut fonts);

        cc.egui_ctx.set_fonts(fonts);

        Self { tx }
    }
}

impl eframe::App for Auria {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Define a theme for egui with catppuccin
        catppuccin_egui::set_theme(&ctx, catppuccin_egui::MACCHIATO);

        ctx.plugin_or_default::<EguiAsyncPlugin>();

        egui::CentralPanel::default().show(ctx, |ui| {
            // Load audio from youtube
            if ui.button("Play").clicked() {
                let url = "https://www.youtube.com/watch?v=ak7OILqZPL4".to_string();
                let _ = self.tx.send(PlayerCommand::Play(url));
            }

            if ui.button("pause").clicked() {
                let _ = self.tx.send(PlayerCommand::Pause);
            }

            if ui.button("resume").clicked() {
                let _ = self.tx.send(PlayerCommand::Resume);
            }
        });
    }
}

async fn audio_task(mut rx: mpsc::UnboundedReceiver<PlayerCommand>) {
    let mut player = AudioPlayer::new();

    while let Some(cmd) = rx.recv().await {
        match cmd {
            PlayerCommand::Play(url) => {
                if let Err(e) = player.play_audio(&url).await {
                    eprintln!("play error: {e}");
                }
            }
            PlayerCommand::Pause => {
                let _ = player.pause();
            }
            PlayerCommand::Resume => {
                let _ = player.resume();
            }
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    // Runtime de Tokio
    let rt = tokio::runtime::Runtime::new().unwrap();

    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

    // Lanza la task de audio
    rt.spawn(audio_task(rx));

    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Auria",
        native_options,
        Box::new(|cc| Ok(Box::new(Auria::new(cc, tx)))),
    );

    Ok(())
}
