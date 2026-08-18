mod game;

use game::{Cell, Game, Outcome, Player};
use gpui::{
    App, Bounds, Context, MouseButton, TitlebarOptions, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, rgb, rgba, size,
};
use gpui_platform::application;

const PAPER: u32 = 0xF6EFE3;
const INK: u32 = 0x2A241C;
const CELL: u32 = 0xFFFDF8;
const GRID: u32 = 0xE2D4C0;
const HOVER: u32 = 0xC9B79A;
const X_RED: u32 = 0xE24B4A;
const O_TEAL: u32 = 0x2F8A78;
const MUTED: u32 = 0x7A6F62;
const WIN_WASH: u32 = 0xF4C15D;

struct TicTacToe {
    game: Game,
}

impl TicTacToe {
    fn status_text(&self) -> String {
        match self.game.outcome() {
            Some(Outcome::Win(Player::X)) => "You win".into(),
            Some(Outcome::Win(Player::O)) => "Computer wins".into(),
            Some(Outcome::Draw) => "Draw".into(),
            None => "Your turn".into(),
        }
    }
}

impl Render for TicTacToe {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let winning = self.game.winning_line().map(|line| line.to_vec());
        let over = self.game.is_over();

        let mut board = div()
            .flex()
            .flex_col()
            .gap(px(8.))
            .p(px(8.))
            .rounded(px(16.))
            .bg(rgb(PAPER));

        for row in 0..3 {
            let mut line = div().flex().flex_row().gap(px(8.));
            for col in 0..3 {
                let index = row * 3 + col;
                let cell = self.game.cell(index);
                let is_win = winning.as_ref().is_some_and(|w| w.contains(&index));
                let mark = match cell {
                    Cell::Occupied(Player::X) => Some(('X', X_RED)),
                    Cell::Occupied(Player::O) => Some(('O', O_TEAL)),
                    Cell::Empty => None,
                };
                let empty = mark.is_none();
                let mut tile = div()
                    .id(("cell", (row * 3 + col) as u32))
                    .size(px(80.))
                    .rounded(px(12.))
                    .border_1()
                    .border_color(rgb(if empty { GRID } else { GRID }))
                    .bg(if is_win {
                        rgba(0xF4C15D59)
                    } else {
                        rgb(CELL).into()
                    })
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(rgb(mark.map(|m| m.1).unwrap_or(INK)))
                    .text_size(px(40.))
                    .child(mark.map(|m| m.0.to_string()).unwrap_or_default());

                if empty && !over && self.game.turn() == Player::X {
                    tile = tile.hover(|s| s.border_color(rgb(HOVER)).cursor_pointer());
                }
                tile = tile.on_mouse_up(
                    MouseButton::Left,
                    cx.listener(move |this, _, _, cx| {
                        this.game.play_at(row, col);
                        cx.notify();
                    }),
                );
                line = line.child(tile);
            }
            board = board.child(line);
        }

        let mut column = div()
            .flex()
            .flex_col()
            .items_center()
            .size_full()
            .bg(rgb(PAPER))
            .p(px(24.))
            .text_color(rgb(INK))
            .child(
                div()
                    .text_size(px(18.))
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .child("Tic-Tac-Toe"),
            )
            .child(
                div()
                    .mt(px(8.))
                    .text_size(px(14.))
                    .text_color(rgb(if over { INK } else { MUTED }))
                    .child(self.status_text()),
            )
            .child(div().mt(px(16.)).child(board));

        if over {
            column = column.child(
                div()
                    .id("reset")
                    .mt(px(20.))
                    .px(px(16.))
                    .py(px(8.))
                    .rounded(px(8.))
                    .bg(rgb(INK))
                    .text_color(rgb(PAPER))
                    .cursor_pointer()
                    .child("Play again")
                    .on_mouse_up(
                        MouseButton::Left,
                        cx.listener(|this, _, _, cx| {
                            this.game.reset();
                            cx.notify();
                        }),
                    ),
            );
        }

        column
    }
}

fn main() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(320.), px(480.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Tic-Tac-Toe".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|_| TicTacToe { game: Game::new() }),
        )
        .unwrap();
        cx.activate(true);
    });
}
