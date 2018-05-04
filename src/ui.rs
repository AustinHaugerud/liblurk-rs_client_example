use tui::Terminal;
use tui::widgets::{BarChart, Block, Borders, Gauge, Item, List, Paragraph, SelectableList, Table,
                   Widget};
use tui::layout::{Direction, Group, Rect, Size};
use tui::style::{Color, Modifier, Style};
use tui::backend::MouseBackend;
use game::GameInformation;

use std::sync::*;

const BORDER_COLOR: Color = Color::Green;
const BACKGROUND_COLOR: Color = Color::Black;
const STANDARD_TEXT_COLOR: Color = Color::White;

pub struct TerminalInterface {
    term: Terminal<MouseBackend>,
    size: Rect,
    game_data: Arc<Mutex<GameInformation>>,
}

impl TerminalInterface {
    pub fn new(game_data: Arc<Mutex<GameInformation>>) -> Result<TerminalInterface, String> {
        let backend = MouseBackend::new()
            .map_err(|_| String::from("Failed to create terminal interface backend."))?;
        let mut term = Terminal::new(backend)
            .map_err(|_| String::from("Failed to create terminal interface."))?;
        let term_size = term.size()
            .map_err(|_| String::from("Failed to get terminal size."))?;
        term.clear()
            .map_err(|_| String::from("Failed to clear terminal."))?;
        term.hide_cursor()
            .map_err(|_| String::from("Failed to hide cursor."))?;

        Ok(TerminalInterface {
            term,
            size: term_size,
            game_data,
        })
    }

    fn base_render(&mut self) {
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(BACKGROUND_COLOR))
            .border_style(Style::default().bg(BACKGROUND_COLOR).fg(BORDER_COLOR))
            .render(&mut self.term, &self.size);
    }

    pub fn render(&mut self) -> Result<(), String> {
        self.base_render();

        let game_info = self.game_data.lock().expect("Failed to lock game data.");

        // It's a lot easier to just clone these guys out
        let messages = game_info.messages.clone();
        let player = game_info.player.clone();

        let message_style = Style::default()
            .bg(BACKGROUND_COLOR)
            .fg(STANDARD_TEXT_COLOR);

        Group::default()
            .direction(Direction::Vertical)
            .sizes(&[
                Size::Percent(10),
                Size::Percent(55),
                Size::Percent(30),
                Size::Percent(5),
            ])
            .render(&mut self.term, &self.size, |term, chunks| {
                // TOP BAR SECTION
                Group::default()
                    .direction(Direction::Horizontal)
                    .sizes(&[Size::Percent(100)])
                    .render(term, &chunks[0], |term, chunks| {
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(BORDER_COLOR).bg(BACKGROUND_COLOR))
                            .style(Style::default().bg(BACKGROUND_COLOR))
                            .render(term, &chunks[0]);
                    });

                // TOP INFO SECTION
                Group::default()
                    .direction(Direction::Horizontal)
                    .sizes(&[Size::Percent(100)])
                    .render(term, &chunks[1], |term, chunks| {
                        Block::default()
                            .borders(Borders::BOTTOM)
                            .border_style(Style::default().bg(BACKGROUND_COLOR).fg(BORDER_COLOR))
                            .style(Style::default().bg(BACKGROUND_COLOR))
                            .render(term, &chunks[0]);
                    });

                // BOTTOM SECTION
                Group::default()
                    .direction(Direction::Horizontal)
                    .sizes(&[Size::Percent(30), Size::Percent(40), Size::Percent(30)])
                    .render(term, &chunks[2], |term, chunks| {
                        // PLAYER SECTION
                        {
                            Group::default()
                                .direction(Direction::Vertical)
                                .sizes(&[Size::Percent(50), Size::Percent(50)])
                                .render(term, &chunks[0], |term, chunks| {
                                    // Stats Hemisphere
                                    {
                                        Group::default()
                                            .direction(Direction::Horizontal)
                                            .sizes(&[Size::Percent(30), Size::Percent(70)])
                                            .render(term, &chunks[0], |term, chunks| {
                                                // Stat Listing
                                                {
                                                    let list_items = vec![
                                                        format!("Health: {}", player.health),
                                                        format!("Gold: {}", player.gold),
                                                        format!("Attack: {}", player.attack),
                                                        format!("Defense: {}", player.defense),
                                                        format!("Regeneration: {}", player.regen),
                                                    ];

                                                    let stats_style = Style::default()
                                                        .fg(STANDARD_TEXT_COLOR)
                                                        .bg(BACKGROUND_COLOR);

                                                    let items = list_items.iter().map(|msg| {
                                                        Item::StyledData(msg, &stats_style)
                                                    });

                                                    List::new(items)
                                                        .block(
                                                            Block::default()
                                                                .border_style(
                                                                    Style::default()
                                                                        .bg(BACKGROUND_COLOR)
                                                                        .fg(BORDER_COLOR),
                                                                )
                                                                .style(
                                                                    Style::default()
                                                                        .bg(BACKGROUND_COLOR),
                                                                )
                                                                .title(player.name.as_str())
                                                                .title_style(
                                                                    Style::default()
                                                                        .bg(BACKGROUND_COLOR)
                                                                        .fg(STANDARD_TEXT_COLOR)
                                                                        .modifier(
                                                                            Modifier::Underline,
                                                                        ),
                                                                ),
                                                        )
                                                        .style(
                                                            Style::default().bg(BACKGROUND_COLOR),
                                                        )
                                                        .render(term, &chunks[0]);
                                                }

                                                // Stat Chart
                                                {
                                                    let mut point_sum = (player.attack
                                                        + player.defense
                                                        + player.regen)
                                                        as f32;

                                                    if point_sum == 0f32 {
                                                        point_sum = 1f32;
                                                    }

                                                    let attack_norm =
                                                        (player.attack as f32 / point_sum) * 100f32;
                                                    let defense_norm = (player.defense as f32
                                                        / point_sum)
                                                        * 100f32;
                                                    let regen_norm =
                                                        (player.regen as f32 / point_sum) * 100f32;

                                                    Group::default()
                                                        .direction(Direction::Vertical)
                                                        .sizes(&[
                                                            Size::Percent(25),
                                                            Size::Percent(25),
                                                            Size::Percent(25),
                                                            Size::Percent(25),
                                                        ])
                                                        .render(
                                                            term,
                                                            &chunks[1],
                                                            |term, chunks| {
                                                                // Attack
                                                                Gauge::default()
                                                                    .label(&format!(
                                                                        "Attack: {}/{}",
                                                                        player.attack, point_sum
                                                                    ))
                                                                    .style(
                                                                        Style::default()
                                                                            .fg(Color::Red)
                                                                            .bg(BACKGROUND_COLOR),
                                                                    )
                                                                    .percent(attack_norm as u16)
                                                                    .render(term, &chunks[0]);

                                                                // Defense
                                                                Gauge::default()
                                                                    .label(&format!(
                                                                        "Defense: {}/{}",
                                                                        player.defense, point_sum
                                                                    ))
                                                                    .style(
                                                                        Style::default()
                                                                            .fg(Color::Cyan)
                                                                            .bg(BACKGROUND_COLOR),
                                                                    )
                                                                    .percent(defense_norm as u16)
                                                                    .render(term, &chunks[1]);

                                                                // Regen
                                                                Gauge::default()
                                                                    .label(&format!(
                                                                        "Regeneration: {}/{}",
                                                                        player.regen, point_sum
                                                                    ))
                                                                    .style(
                                                                        Style::default()
                                                                            .fg(Color::LightGreen)
                                                                            .bg(BACKGROUND_COLOR),
                                                                    )
                                                                    .percent(regen_norm as u16)
                                                                    .render(term, &chunks[2]);

                                                                Paragraph::default()
                                                                    .text(&format!(
                                                                        "Total Points: {}",
                                                                        point_sum as u32
                                                                    ))
                                                                    .style(
                                                                        Style::default()
                                                                            .bg(BACKGROUND_COLOR)
                                                                            .fg(
                                                                                STANDARD_TEXT_COLOR,
                                                                            ),
                                                                    )
                                                                    .render(term, &chunks[3]);
                                                            },
                                                        );
                                                }
                                            });
                                    }

                                    // Description Hemisphere
                                    {
                                        Paragraph::default()
                                            .style(
                                                Style::default()
                                                    .bg(BACKGROUND_COLOR)
                                                    .fg(STANDARD_TEXT_COLOR)
                                                    .modifier(Modifier::Italic),
                                            )
                                            .block(
                                                Block::default()
                                                    .borders(Borders::TOP)
                                                    .border_style(
                                                        Style::default()
                                                            .bg(BACKGROUND_COLOR)
                                                            .fg(BORDER_COLOR),
                                                    ),
                                            )
                                            .text(&player.description)
                                            .render(term, &chunks[1]);
                                    }
                                });
                        }

                        // MESSAGE FEED
                        {
                            let items = messages.iter().map(|msg| {
                                Item::StyledData(
                                    format!("{}: {}", msg.sender, msg.content),
                                    &message_style,
                                )
                            });
                            List::new(items)
                                .block(
                                    Block::default()
                                        .border_style(
                                            Style::default().bg(BACKGROUND_COLOR).fg(BORDER_COLOR),
                                        )
                                        .title("Message Feed")
                                        .title_style(
                                            Style::default()
                                                .bg(BACKGROUND_COLOR)
                                                .fg(STANDARD_TEXT_COLOR)
                                                .modifier(Modifier::Underline),
                                        )
                                        .style(Style::default().bg(BACKGROUND_COLOR))
                                        .borders(Borders::LEFT | Borders::RIGHT),
                                )
                                .style(Style::default().bg(BACKGROUND_COLOR))
                                .render(term, &chunks[1]);
                        }

                        // ENTITY VIEW SECTION
                        {
                            Block::default()
                                .border_style(
                                    Style::default().bg(BACKGROUND_COLOR).fg(BORDER_COLOR),
                                )
                                .style(Style::default().bg(BACKGROUND_COLOR))
                                .render(term, &chunks[2]);
                        }
                    });

                // INPUT SECTION
                Group::default()
                    .direction(Direction::Horizontal)
                    .sizes(&[Size::Percent(100)])
                    .render(term, &chunks[3], |term, chunks| {
                        Block::default()
                            .border_style(Style::default().bg(BACKGROUND_COLOR).fg(BORDER_COLOR))
                            .style(Style::default().bg(BACKGROUND_COLOR))
                            .borders(Borders::ALL)
                            .render(term, &chunks[0]);
                    });
            });

        self.term
            .draw()
            .map_err(|_| String::from("Failed to render."))?;
        Ok(())
    }

    pub fn show_cursor(&mut self) -> Result<(), ()> {
        self.term.show_cursor().map_err(|_| ())
    }

    pub fn update(&mut self) -> Result<(), String> {
        let size = self.term
            .size()
            .map_err(|_| String::from("Failed to to see new terminal size in update."))?;
        if size != self.size {
            self.term
                .resize(size)
                .map_err(|_| String::from("Failed to resize terminal."))?;
            self.size = size;
        }
        Ok(())
    }
}

impl Drop for TerminalInterface {
    fn drop(&mut self) {
        self.show_cursor().expect("Failed to reshow cursor.");
    }
}
