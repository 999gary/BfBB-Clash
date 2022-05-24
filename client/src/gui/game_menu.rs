use std::collections::HashMap;

use clash::{game_state::{GameState, SpatulaState}, player::SharedPlayer, spatula::Spatula, PlayerId};
use eframe::{
    egui::{Color32, Response, Sense, Ui, Widget},
    epaint::Vec2,
};
use strum::IntoEnumIterator;

pub struct GameMenu<'a> {
    game: &'a GameState,
    players: &'a HashMap<PlayerId, SharedPlayer>,
}

impl<'a> GameMenu<'a> {
    pub fn new(game: &'a GameState, players: &'a HashMap<PlayerId, SharedPlayer>) -> Self {
        Self { game, players }
    }
}
impl<'a> Widget for GameMenu<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let available_size = ui.available_size();

        // Determine largest radius without overflowing bounds
        let width_radius = (available_size.x - 8. * 7.) / 8. / 2.;
        let height_radius = (available_size.y - 8. * 12.) / 13. / 2.;
        let radius = f32::min(width_radius, height_radius);
        let desired_size = Vec2::new(radius * 2. * 8. + (8. * 7.), radius * 2. * 13. + (8. * 12.));

        let (rect, response) =
            ui.allocate_exact_size(desired_size, Sense::focusable_noninteractive());

        for spat in Spatula::iter() {
            let mut spat_state = self.game.spatulas.get(&spat).unwrap_or(&SpatulaState::default()).clone();
            let color = spat_state.tier.get_color();
            let color = Color32::from_rgb(color.0, color.1, color.2);
            let (y, x) = spat.into();
            ui.painter().circle_filled(
                (
                    x as f32 * (2. * radius + 8.) + rect.left_top().x + radius,
                    y as f32 * (2. * radius + 8.) + rect.left_top().y + radius,
                )
                    .into(),
                radius,
                color,
            )
        }

        response
    }
}
