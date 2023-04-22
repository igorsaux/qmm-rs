use std::{
    fmt::Debug,
    io::{self, Stdout},
    time::Duration,
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use qmm_player::{QuestPlayer, QuestState};
use qmm_syntax::text::formatted_text::{FormattedText, TextElement, TextElementKind};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};

type OptionControlCallback = fn(&OptionControl, &mut CliQuestPlayer);

#[derive(Debug, Clone)]
enum PlayerState {
    PreStart,
    InGame { state: QuestState },
    Exit,
}

#[derive(Clone)]
pub struct OptionControl {
    pub name: FormattedText,
    on_selected: Option<OptionControlCallback>,
}

impl Debug for OptionControl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OptionControl")
            .field("name", &self.name)
            .finish()
    }
}

impl OptionControl {
    pub fn new(name: &str, on_selected: Option<OptionControlCallback>) -> OptionControl {
        Self {
            name: FormattedText::parse(name),
            on_selected,
        }
    }

    pub fn selected(&self, player: &mut CliQuestPlayer) {
        let Some(callback) = self.on_selected else {
            return;
        };

        callback(self, player);
    }
}

#[derive(Debug, Clone)]
pub struct CliQuestPlayer<'q> {
    player: QuestPlayer<'q>,
    state: PlayerState,
    selected_option: usize,
    options: Vec<OptionControl>,
}

fn conv_formatted_text(text: FormattedText) -> Text<'static> {
    let mut result_text = Text::default();
    let text_style = Style::default()
        .fg(Color::LightBlue)
        .add_modifier(Modifier::BOLD);

    let mut spans = Vec::new();
    for el in text.elements {
        match el.kind {
            TextElementKind::NewLine => {
                result_text.extend(Text::from(Spans::from(spans)));
                spans = Vec::new();
            }
            TextElementKind::Variable { .. } => spans.push(Span::styled(el.value, text_style)),
            TextElementKind::Selection { text } => spans.push(Span::styled(text, text_style)),
            _ => spans.push(Span::raw(el.value)),
        }
    }

    result_text.extend(Text::from(Spans::from(spans)));
    result_text
}

impl<'q> CliQuestPlayer<'q> {
    pub fn new(player: QuestPlayer<'q>) -> Self {
        Self {
            player,
            state: PlayerState::PreStart,
            selected_option: 0,
            options: Vec::new(),
        }
    }

    pub fn set_options(&mut self, options: Vec<OptionControl>) {
        self.selected_option = 0;
        self.options = options;
    }

    fn on_start_selected(_: &OptionControl, player: &mut CliQuestPlayer) {
        let state = player.player.state().clone();
        player.set_options(
            state
                .jumps
                .iter()
                .map(|jump| OptionControl {
                    name: jump.name.clone(),
                    on_selected: None,
                })
                .collect(),
        );
        player.state = PlayerState::InGame { state };
    }

    fn on_exit_selected(_: &OptionControl, player: &mut CliQuestPlayer) {
        player.state = PlayerState::Exit;
    }

    pub fn run(mut self) {
        enable_raw_mode().unwrap();
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();

        self.set_options(vec![
            OptionControl::new("Start", Some(Self::on_start_selected)),
            OptionControl::new("Exit", Some(Self::on_exit_selected)),
        ]);
        self.play(&mut terminal);

        disable_raw_mode().unwrap();
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .unwrap();
        terminal.show_cursor().unwrap();
    }

    fn play(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
        // Flush events
        while let Ok(true) = event::poll(Duration::from_millis(250)) {
            event::read().ok();
        }

        loop {
            if matches!(self.state, PlayerState::Exit) {
                return;
            }

            terminal.draw(|frame| self.ui(frame)).unwrap();

            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Esc | KeyCode::Char('Q') => return,
                    KeyCode::Up => {
                        self.selected_option = self.selected_option.saturating_sub(1);
                    }
                    KeyCode::Down => {
                        self.selected_option = self.selected_option.saturating_add(1);
                    }
                    KeyCode::Enter => {
                        if let Some(option) = self.options.get(self.selected_option) {
                            unsafe {
                                let player = self as *const CliQuestPlayer as *mut CliQuestPlayer;
                                option.selected(&mut *player);
                            }
                        }
                    }
                    _ => (),
                }
            }

            self.selected_option = self
                .selected_option
                .clamp(0, self.options.len().wrapping_sub(1));
        }
    }

    fn ui(&mut self, frame: &mut Frame<CrosstermBackend<Stdout>>) {
        let player = &self.player;
        let size = frame.size();

        // Main layout
        let term_layout = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(size);

        // Right bar layout
        let right_bar_layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
            .split(term_layout[1]);

        // Params block
        let params_block = Block::default()
            .borders(Borders::ALL)
            .title("Info")
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Double);

        frame.render_widget(params_block, right_bar_layout[0]);

        // Help block
        let help_paragragh = Paragraph::new("ESC/Q - exit").block(
            Block::default()
                .borders(Borders::ALL)
                .title("Keys")
                .title_alignment(Alignment::Left)
                .border_type(BorderType::Double),
        );

        frame.render_widget(help_paragragh, right_bar_layout[1]);

        let main_block = Block::default()
            .title("Quest Player")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Double);

        frame.render_widget(main_block, term_layout[0]);

        let main_layout = Layout::default()
            .margin(1)
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(70), Constraint::Min(30)])
            .split(term_layout[0]);

        match &self.state {
            PlayerState::PreStart => {
                let task_text = player.task_text().clone();
                let text_block =
                    Paragraph::new(conv_formatted_text(task_text)).wrap(Wrap { trim: true });

                frame.render_widget(text_block, main_layout[0]);
            }
            PlayerState::InGame { state } => {
                let location_text_block =
                    Paragraph::new(conv_formatted_text(state.location.description.clone()))
                        .wrap(Wrap { trim: true });

                frame.render_widget(location_text_block, main_layout[0]);
            }
            PlayerState::Exit => return,
        }

        let items: Vec<ListItem> = self
            .options
            .iter()
            .enumerate()
            .map(|(idx, option)| {
                let mut option_name = FormattedText {
                    elements: vec![TextElement {
                        kind: TextElementKind::Text,
                        value: if self.selected_option == idx {
                            "> ".to_string()
                        } else {
                            "  ".to_string()
                        },
                    }],
                };

                option_name.elements.extend(option.name.elements.clone());

                let style = if self.selected_option == idx {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default()
                };

                let mut text = conv_formatted_text(option_name);
                text.patch_style(style);

                ListItem::new(text)
            })
            .collect();

        let input_block = List::new(items).block(
            Block::default()
                .borders(Borders::TOP)
                .border_type(BorderType::Rounded),
        );

        frame.render_widget(input_block, main_layout[1]);
    }
}
