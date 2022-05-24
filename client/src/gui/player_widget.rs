use std::ops::Index;

use clash::{player::SharedPlayer, room::Room, game_state::GameState};
use eframe::egui::{Response, Sense, Stroke, TextStyle, Ui, Vec2, Widget};

pub struct PlayerUi<'a> {
    player: &'a SharedPlayer,
    score: u32,
}

impl<'a> PlayerUi<'a> {
    pub fn new(player: &'a SharedPlayer, score: u32) -> Self {
        Self { player, score }
    }
}

impl<'a> Widget for PlayerUi<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let PlayerUi { player, score } = self;
        let color = player.options.color();

        // Use individual layouts instead of a single one to be able to add padding between each line
        // TODO: Look into how to avoid these constant string allocations
        //       Probably store these Galleys in a client-side player struct
        let name_galley = ui.painter().layout_no_wrap(
            player.options.name.clone(),
            TextStyle::Body.resolve(ui.style()),
            color,
        );
        let score_galley = ui.painter().layout_no_wrap(
            format!("Score: {}", score),
            TextStyle::Body.resolve(ui.style()),
            color,
        );
        let room_galley = ui.painter().layout_no_wrap(
            player
                .current_room
                .map(|r| r.to_string())
                .unwrap_or_else(|| "? ? ?".to_string()),
            TextStyle::Small.resolve(ui.style()),
            color,
        );

        let name_size = name_galley.size();
        let score_size = score_galley.size();
        let room_size = room_galley.size();
        // Use longest level name for the overall width
        let longest_width = ui
            .painter()
            .layout_no_wrap(
                Room::MermalairVillianContainment.to_string(),
                TextStyle::Small.resolve(ui.style()),
                color,
            )
            .size()
            .x;

        let padding = ui.spacing().button_padding;
        let desired_size = Vec2::new(
            longest_width + 4. * padding.x,
            name_size.y + score_size.y + room_size.y + 4. * padding.y,
        );
        let (rect, response) =
            ui.allocate_exact_size(desired_size, Sense::focusable_noninteractive());

        if ui.is_rect_visible(rect) {
            ui.painter().rect_stroke(rect, 0., Stroke::new(2., color));
            let mut text_pos = rect.left_top() + padding;

            ui.painter().galley(text_pos, name_galley);
            text_pos.y += name_size.y + padding.y;

            ui.painter().galley(text_pos, score_galley);
            text_pos.y += score_size.y + padding.y;

            ui.painter().galley(text_pos, room_galley);
        }

        response
    }
}
