use std::collections::HashSet;

use egui::{FontData, FontDefinitions, FontFamily};
use egui_async::{Bind, EguiAsyncPlugin};
use tokio::sync::mpsc;

use crate::player::{
    player::AudioPlayer,
    search::{VideoFormat, search_videos},
};

mod player;

#[derive(Clone)]
enum PlayerCommand {
    Play(String),
    Pause,
    Resume,
    Search(String),
}

struct Auria {
    //audio_bind: Bind<(), String>,
    tx: mpsc::UnboundedSender<PlayerCommand>,
    search: String,
    video_bind: Bind<HashSet<VideoFormat>, String>,
}

impl Auria {
    fn new(cc: &eframe::CreationContext, tx: mpsc::UnboundedSender<PlayerCommand>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        let mut fonts = FontDefinitions::default();
        // Install my own font (maybe supporting non-latin characters):
        fonts.font_data.insert(
            "my_font".to_owned(),
            std::sync::Arc::new(
                // .ttf and .otf supported
                FontData::from_static(include_bytes!(
                    "../assets/fonts/NotoSansJP-VariableFont_wght.ttf"
                )),
            ),
        );

        // Put my font first (highest priority):
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "my_font".to_owned());

        // Put my font as last fallback for monospace:
        fonts
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .push("my_font".to_owned());

        cc.egui_ctx.set_fonts(fonts);

        Self {
            tx,
            search: String::new(),
            video_bind: Bind::default(),
        }
    }
}

impl eframe::App for Auria {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.options_mut(|options| {
            options.max_passes = std::num::NonZeroUsize::new(2).unwrap();
        });

        ctx.plugin_or_default::<EguiAsyncPlugin>();

        let Self {
            tx,
            search,
            video_bind,
        } = self;

        // Define a theme for egui with catppuccin
        catppuccin_egui::set_theme(&ctx, catppuccin_egui::MACCHIATO);

        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            let search_bar = ui.text_edit_singleline(search);

            if !search.is_empty() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                egui::Window::new("results")
                    .title_bar(false)
                    .fixed_pos(search_bar.rect.left_bottom())
                    .show(ctx, |_ui| {
                        let query = search.clone();
                        video_bind.request(async move {
                            search_videos(query.to_string())
                                .await
                                .map_err(|e| e.to_string())
                        });

                        let _ = tx.send(PlayerCommand::Search(search.to_string()));
                        println!("searching {}", search);
                    });
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(res) = video_bind.read() {
                match res {
                    Ok(videos) => {
                        for video in videos.iter() {
                            if ui.button(&video.title).clicked() {
                                let _ = tx.send(PlayerCommand::Play(video.url.to_string()));
                            }
                        }
                    }
                    Err(err) => {
                        ui.colored_label(
                            egui::Color32::RED,
                            format!("Could not fetch video.\nError: {err}"),
                        );
                    }
                }
            } else {
                ui.label("getting the song");
                ui.spinner();
            }
        });

        egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
            if ui.button("pause").clicked() {
                let _ = tx.send(PlayerCommand::Pause);
            }

            if ui.button("resume").clicked() {
                let _ = tx.send(PlayerCommand::Resume);
            }
        });

        egui::SidePanel::right("side panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.label("side panel");
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
            PlayerCommand::Search(query) => {
                let videos = search_videos(query).await.unwrap();
                println!("{:?}", videos);
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
