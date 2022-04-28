mod game_menu;
mod player_widget;

use crate::game::GameStateExt;

use self::{game_menu::GameMenu, player_widget::PlayerUi};
use clash::lobby::{GamePhase, LobbyOptions, SharedLobby};
use clash::player::PlayerOptions;
use clash::protocol::Message;
use clash::PlayerId;
use std::error::Error;
use std::sync::mpsc::Receiver;

use eframe::egui::{
    Align, Area, Button, CentralPanel, Checkbox, Color32, Context, FontData, FontDefinitions,
    FontFamily, Label, Layout, RichText, SidePanel, Style, TextEdit, TextStyle, TopBottomPanel, Ui,
};
use eframe::epaint::{ColorImage, FontId, Pos2, TextureHandle};
use eframe::epi::{App, Frame, IconData, Storage};
use eframe::{run_native, NativeOptions};

const BORDER: f32 = 32.;
const PADDING: f32 = 8.;

pub enum Menu {
    Main,
    Host,
    Join,
    Game,
}

pub struct Clash {
    gui_receiver: Receiver<(PlayerId, SharedLobby)>,
    network_sender: tokio::sync::mpsc::Sender<Message>,
    error_receiver: Receiver<Box<dyn Error + Send>>,

    state: Menu,
    name_buf: String,

    logo: Option<TextureHandle>,

    lobby_id_buf: String,
    lobby_id: Option<u32>,

    lab_door_buf: String,
    lab_door_num: Option<u8>,

    player_id: PlayerId,
    lobby: SharedLobby,

    error_queue: Vec<Box<dyn Error>>,
}

impl Clash {
    fn new(
        gui_receiver: Receiver<(PlayerId, SharedLobby)>,
        error_receiver: Receiver<Box<dyn Error + Send>>,
        network_sender: tokio::sync::mpsc::Sender<Message>,
    ) -> Self {
        Self {
            gui_receiver,
            network_sender,
            error_receiver,
            logo: None,
            state: Menu::Main,
            name_buf: Default::default(),
            lobby_id_buf: Default::default(),
            lobby_id: Default::default(),
            lab_door_buf: Default::default(),
            lab_door_num: None,
            player_id: 0,
            lobby: SharedLobby::new(0, LobbyOptions::default()),
            error_queue: Vec::new(),
        }
    }

    fn paint_options(&mut self, ui: &mut Ui) {
        ui.heading("Lobby Options");
        ui.separator();

        ui.add_enabled_ui(self.lobby.host_id == Some(self.player_id), |ui| {
            let ng_plus_response = ui
                .add(Checkbox::new(&mut self.lobby.options.ng_plus, "New Game+"))
                .on_hover_text(
                    "All players start the game with the Bubble Bowl and Cruise Missile unlocked.",
                );

            let door_cost_response = ui
                .horizontal(|ui| {
                    if self.lab_door_num.is_none() {
                        ui.style_mut().visuals.override_text_color = Some(Color32::DARK_RED);
                    }
                    ui.label("Lab Door Cost: ");
                    ui.text_edit_singleline(&mut self.lab_door_buf)
                })
                .inner;

            if !ui.is_enabled() {
                return;
            }

            // Validate input
            let mut door_cost_change_valid = false;
            if door_cost_response.changed() {
                self.lab_door_num = self
                    .lab_door_buf
                    .parse::<u8>()
                    .ok()
                    .filter(|&n| n > 0 && n <= 82);
                if let Some(n) = self.lab_door_num {
                    self.lobby.options.lab_door_cost = n;
                    door_cost_change_valid = true;
                }
            }

            if door_cost_change_valid || ng_plus_response.changed() {
                self.network_sender
                    .blocking_send(Message::GameOptions {
                        options: self.lobby.options.clone(),
                    })
                    .unwrap();
            }

            let mut start_game_response = ui
                .add_enabled(
                    self.lobby.can_start() && self.lab_door_num.is_some(),
                    Button::new("Start Game"),
                )
                .on_hover_text("Starts a new game for all connected players.");

            // We unfortunately have to check these conditions twice since we need the Response to add the
            // tooltips but need to enable/disable the button before we can get the response
            if !self.lobby.can_start() {
                start_game_response = start_game_response
                    .on_disabled_hover_text("All players must be on the Main Menu.")
            }

            if self.lab_door_num.is_none() {
                start_game_response = start_game_response
                    .on_disabled_hover_text("'Lab Door Cost' must be a number from 1-82");
            }

            if start_game_response.clicked() {
                // TODO: Send a message to the network thread to start the game.
                self.network_sender
                    .blocking_send(Message::GameBegin {})
                    .unwrap();
            }
        });
    }
}

impl App for Clash {
    fn setup(&mut self, ctx: &Context, _frame: &Frame, _storage: Option<&dyn Storage>) {
        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert(
            "spongebob".into(),
            // TODO: include_bytes is platform specific, this will not compile on linux.
            FontData::from_static(include_bytes!("..\\..\\fonts\\Some.Time.Later.otf")),
        );
        font_def
            .families
            .insert(FontFamily::Proportional, vec!["spongebob".into()]);

        ctx.set_fonts(font_def);

        //
        let mut style = Style::default();
        style.spacing.button_padding = (PADDING, PADDING).into();
        style.spacing.item_spacing = (PADDING, PADDING).into();
        style.text_styles.insert(
            TextStyle::Heading,
            FontId {
                size: 42.,
                family: FontFamily::Proportional,
            },
        );
        style.text_styles.insert(
            TextStyle::Body,
            FontId {
                size: 32.,
                family: FontFamily::Proportional,
            },
        );
        style.text_styles.insert(
            TextStyle::Small,
            FontId {
                size: 18.,
                family: FontFamily::Proportional,
            },
        );
        style.text_styles.insert(
            TextStyle::Button,
            FontId {
                size: 40.,
                family: FontFamily::Proportional,
            },
        );

        ctx.set_style(style);
    }

    fn update(&mut self, ctx: &Context, frame: &Frame) {
        let style = ctx.style();
        let height = ctx.fonts().row_height(&TextStyle::Small.resolve(&style));

        // Paint this at the end so it stays on top
        let version_ui = Area::new("Version")
            // TODO: Find how to not hardcode this
            .fixed_pos(Pos2::new(560., ctx.available_rect().height() - height));

        while let Ok(e) = self.error_receiver.try_recv() {
            self.error_queue.push(e);
        }
        TopBottomPanel::bottom("errors").show(ctx, |ui| {
            if let Some(e) = self.error_queue.get(0) {
                ui.add(Label::new(
                    RichText::new(format!("Error!: {e}")).color(Color32::DARK_RED),
                ));
                if ui.button("OK").clicked() {
                    self.error_queue.remove(0);
                }
            }
        });

        match self.state {
            Menu::Main => {
                CentralPanel::default().show(ctx, |ui| {
                    ui.with_layout(Layout::top_down(Align::Center), |ui| {
                        let texture: &TextureHandle = self.logo.get_or_insert_with(|| {
                            ctx.load_texture(
                                "logo",
                                load_image_from_memory(include_bytes!("../../res/logo.png"))
                                    .unwrap(),
                            )
                        });
                        ui.image(texture, texture.size_vec2() / 3.);
                    });
                    ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                        ui.add_space(BORDER);
                        if ui.button("Quit").clicked() {
                            frame.quit();
                        }
                        if ui.button("Join Game").clicked() {
                            self.state = Menu::Join;
                        }
                        if ui.button("Host Game").clicked() {
                            self.state = Menu::Host;
                        }
                    });
                });
            }
            Menu::Host => {
                TopBottomPanel::top("Title").show(ctx, |ui| {
                    ui.vertical_centered(|ui| ui.label("Host Game"));
                });
                TopBottomPanel::bottom("Join Panel").show(ctx, |ui| {
                    ui.add(TextEdit::singleline(&mut self.name_buf).hint_text("Name"));
                    ui.add_enabled_ui(!self.name_buf.is_empty(), |ui| {
                        if ui.button("Host Game").clicked() {
                            self.network_sender
                                .blocking_send(Message::GameHost)
                                .unwrap();
                            self.network_sender
                                .blocking_send(Message::PlayerOptions {
                                    options: PlayerOptions {
                                        name: self.name_buf.clone(),
                                        color: (0, 0, 0),
                                    },
                                })
                                .unwrap();

                            self.state = Menu::Game;
                        }
                    });
                    if ui.button("Back").clicked() {
                        self.state = Menu::Main;
                    }
                    ui.add_space(BORDER);
                });
            }
            Menu::Join => {
                TopBottomPanel::top("Title").show(ctx, |ui| {
                    ui.label("Join Game");
                });
                TopBottomPanel::bottom("Host Panel").show(ctx, |ui| {
                    ui.add(TextEdit::singleline(&mut self.name_buf).hint_text("Name"));
                    let lobby_response = ui.add(
                        TextEdit::singleline(&mut self.lobby_id_buf)
                            .hint_text("Lobby ID")
                            .password(true),
                    );

                    // Validate input
                    if lobby_response.changed() {
                        self.lobby_id = u32::from_str_radix(self.lobby_id_buf.as_str(), 16).ok();
                    }

                    let mut join_response = ui
                        .add_enabled_ui(self.lobby_id.is_some(), |ui| ui.button("Join Game"))
                        .inner;

                    if self.lobby_id.is_none() {
                        join_response = join_response.on_disabled_hover_text(
                            "Lobby ID must be an 8 digit hexadecimal number",
                        )
                    };

                    if join_response.clicked() {
                        self.network_sender
                            .blocking_send(Message::GameJoin {
                                lobby_id: self.lobby_id.unwrap(),
                            })
                            .unwrap();
                        self.network_sender
                            .blocking_send(Message::PlayerOptions {
                                options: PlayerOptions {
                                    name: self.name_buf.clone(),
                                    color: (0, 0, 0),
                                },
                            })
                            .unwrap();
                        self.state = Menu::Game;
                    }

                    if ui.button("Back").clicked() {
                        self.state = Menu::Main;
                    }
                    ui.add_space(BORDER);
                });
            }
            Menu::Game => {
                // Continuously repaint
                ctx.request_repaint();

                // Receive gamestate updates
                while let Ok((local_player_id, new_lobby)) = self.gui_receiver.try_recv() {
                    self.player_id = local_player_id;
                    self.lab_door_buf = new_lobby.options.lab_door_cost.to_string();
                    self.lab_door_num = Some(new_lobby.options.lab_door_cost);
                    self.lobby = new_lobby;
                }

                SidePanel::left("Player List")
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.add_space(PADDING);
                        // TODO: Cache this
                        let mut values = self.lobby.players.values().collect::<Vec<_>>();
                        values.sort_by(|&a, &b| a.menu_order.cmp(&b.menu_order));
                        for player in values {
                            ui.add(PlayerUi::new(player));
                        }
                    });
                CentralPanel::default().show(ctx, |ui| {
                    match self.lobby.game_phase {
                        GamePhase::Setup => {
                            self.paint_options(ui);
                            if ui.button("Copy Lobby ID").clicked() {
                                ctx.output().copied_text = format!("{:X}", self.lobby.lobby_id);
                            }
                        }
                        GamePhase::Playing => {
                            ui.add(GameMenu::new(&self.lobby.game_state, &self.lobby.players));
                        }
                        GamePhase::Finished => todo!(),
                    }
                    ui.with_layout(Layout::bottom_up(Align::LEFT), |ui| {
                        if ui.button("Leave").clicked() {
                            let _ = self.network_sender.blocking_send(Message::GameLeave);
                            self.state = Menu::Main;
                        }
                    })
                });
            }
        }

        version_ui.show(ctx, |ui| {
            ui.small(crate::VERSION);
        });
    }

    fn name(&self) -> &str {
        "BfBB Clash"
    }
}

pub fn run(
    gui_receiver: Receiver<(PlayerId, SharedLobby)>,
    error_receiver: Receiver<Box<dyn Error + Send>>,
    network_sender: tokio::sync::mpsc::Sender<Message>,
) {
    let icon_bytes = include_bytes!("../../res/icon.ico");
    let icon = image::load_from_memory(icon_bytes).unwrap().to_rgba8();
    let (width, height) = icon.dimensions();

    let window_options = NativeOptions {
        initial_window_size: Some((600., 742.).into()),
        resizable: false,
        icon_data: Some(IconData {
            rgba: icon.into_raw(),
            width,
            height,
        }),
        ..Default::default()
    };

    run_native(
        Box::new(Clash::new(gui_receiver, error_receiver, network_sender)),
        window_options,
    );
}

fn load_image_from_memory(image_data: &[u8]) -> Result<ColorImage, image::ImageError> {
    let image = image::load_from_memory(image_data)?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()))
}
