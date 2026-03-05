use crate::stylesheet::get_stylesheet;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Padding, Paragraph};

#[derive(Clone, Copy)]
enum AgentStatus {
    READY,
    WORKING,
    WAITING,
    BLOCKED,
}

struct AgentStatusRow {
    name: String,
    status: AgentStatus,
}

pub struct StatusWidget {
    agents: Vec<AgentStatusRow>,
}

impl StatusWidget {
    pub fn new() -> Self {
        let names = vec![
            "Coordinator", "Spec Writer", "Spec Verifier", "Test Writer", "Implementor", "Adversary"
        ];
        let r = AgentStatus::READY;
        let w = AgentStatus::WORKING;
        let a = AgentStatus::WAITING;
        let b = AgentStatus::BLOCKED;
        let stats = vec![ a, r, w, w, a, a ];
        Self {
            agents: std::iter::zip(names, stats).map(|(n, s)| AgentStatusRow { name: n.to_string(), status: s, }).collect(),
        }
    }
}

fn line_of_agent_status_row(row: &AgentStatusRow) -> Line<'static> {
    match row.status {
        AgentStatus::READY => Line::from(vec![Span::raw("* "), Span::raw(row.name.clone())]), //format!("{} {}", "*", row.name)),
        _ => Line::raw("blah"),
    }
}

impl Widget for &StatusWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let styles = get_stylesheet();
        let red = Style::new().red();
        let mut lines = vec![];

        for row in &self.agents {
            lines.push(line_of_agent_status_row(&row));
        }

        let outer_block = Block::new()
            .padding(Padding::symmetric(2, 1))
            .style(styles.status());
        let status_area = Paragraph::new(lines).block(
            Block::new()
                .style(styles.status())
                .borders(Borders::NONE)
                .padding(Padding::symmetric(0, 1))
                .title(Span::raw("Status").style(styles.status_title())),
        );
        let inner_area = outer_block.inner(area);
        outer_block.render(area, buf);
        status_area.render(inner_area, buf);
    }
}
