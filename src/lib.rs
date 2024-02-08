use ratatui::{
    layout::Flex,
    prelude::*,
    widgets::{Block, Paragraph, Row, Table, TableState},
};
use ratatui_macros::{constraints, horizontal, vertical};
use tui_big_text::{BigTextBuilder, PixelSize};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Application.
#[derive(Debug, Default, Clone)]
pub struct Dashboard<'a> {
    general_title: String,
    subtitle: String,
    avatar: String,
    avatar_block: Option<Block<'a>>,
    table: Vec<TableItem>,
    table_block: Option<Block<'a>>,
    footer: Vec<String>,
}

#[derive(Debug, Default, Clone)]
pub struct TableItem {
    left: String,
    right: String,
}

impl TableItem {
    pub fn new<I: Into<String>>(i: (I, I)) -> Self {
        Self {
            left: i.0.into(),
            right: i.1.into(),
        }
    }
}

impl<T> From<(T, T)> for TableItem
where
    T: Into<String>,
{
    fn from(value: (T, T)) -> Self {
        Self::new(value)
    }
}

#[derive(Default)]
pub struct DashboardBuilder<'a>(Dashboard<'a>);

impl<'a> DashboardBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn general_title(mut self, general_title: impl Into<String>) -> Self {
        self.0.general_title = general_title.into();
        self
    }

    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.0.subtitle = subtitle.into();
        self
    }

    pub fn avatar(mut self, avatar: impl Into<String>) -> Self {
        self.0.avatar = avatar.into();
        self
    }

    pub fn avatar_block(mut self, block: impl Into<Option<Block<'a>>>) -> Self {
        self.0.avatar_block = block.into();
        self
    }

    pub fn table(mut self, table: Vec<impl Into<TableItem>>) -> Self {
        self.0.table = table.into_iter().map(|i| i.into()).collect();
        self
    }

    pub fn table_block(mut self, block: impl Into<Option<Block<'a>>>) -> Self {
        self.0.table_block = block.into();
        self
    }

    pub fn footer(mut self, footer: Vec<impl Into<String>>) -> Self {
        self.0.footer = footer.into_iter().map(|i| i.into()).collect();
        self
    }

    pub fn build(self) -> Dashboard<'a> {
        // let header_area = vertical![==20%, ==70%, ==5%].split()
        self.0
    }
}

impl Dashboard<'_> {
    fn render_header(&self, area: Rect, buf: &mut Buffer) {
        let [top_1, top_2] = vertical![==10%, ==5%]
            .vertical_margin(1)
            .split(area)
            .to_vec()
            .try_into()
            .unwrap();

        let top_center_1 = horizontal![==(self.general_title.chars().count() as u16 * 4)]
            .flex(Flex::Center)
            .split(top_1)[0];

        let top_center_2 = horizontal![==(self.subtitle.chars().count() as u16)]
            .flex(Flex::Center)
            .split(top_2)[0];

        let title = Line::raw(&self.general_title).style(Style::new().light_red());
        let title = BigTextBuilder::default()
            .pixel_size(PixelSize::Quadrant)
            .lines(vec![title])
            .build()
            .unwrap();

        let subtitle = Line::raw(&self.subtitle).style(Style::new().blue().bold());
        title.render(top_center_1, buf);
        subtitle.render(top_center_2, buf);
    }

    fn render_table<'a, 'b: 'a>(&'b mut self, area: Rect, buf: &mut Buffer, state: &mut TableState) {
        let [_, main] = horizontal![==36%, ==57%]
            .horizontal_margin(3)
            .split(area)
            .to_vec()
            .try_into()
            .unwrap();

        let lines = {
            let to_line = |(idx, a): (usize, &'a TableItem)| {
                let style = match state.selected() {
                    Some(i) if i == idx => Style::default().italic().bold().underlined(),
                    _ => Style::default(),
                };
                let description = a.left.as_str().blue().style(style);
                let keyboard = a.right.as_str().to_uppercase().light_red().bold();
                Line::default().spans(vec![description, keyboard]).style(style)
            };
            self.table.iter().enumerate().map(to_line)
        };

        let mut table_width = 0;
        let mut table_height = 0;
        for i in lines.clone() {
            table_width = table_width.max(i.width() + 2);
            table_height += 1;
        }

        let table = {
            let rows = lines.map(Row::new);
            let widths = constraints![==95%, ==5%];
            let block = self.table_block.clone().unwrap_or(Block::bordered());
            Table::new(rows, widths).highlight_symbol(" >> ").cyan().block(block)
        };

        let [_, main] = vertical![==20%, ==table_height + 2].split(main).to_vec().try_into().unwrap();
        StatefulWidget::render(table, main, buf, state);
    }

    fn render_avatar(&self, area: Rect, buf: &mut Buffer) {
        let lines = self.avatar.lines().map(|i| Line::from(i.light_blue())).collect::<Vec<_>>();
        let mut avatar_width = 0;
        let mut avatar_height = 0;
        for line in lines.iter() {
            avatar_width = avatar_width.max(line.width());
            avatar_height += 1;
        }
        let [_, left] = horizontal![==8%, ==avatar_width as u16]
            .split(area)
            .to_vec()
            .try_into()
            .unwrap();
        let [_, left] = vertical![==20%, ==avatar_height as u16]
            .split(left)
            .to_vec()
            .try_into()
            .unwrap();
        let block = self.avatar_block.clone().unwrap_or(Block::bordered());
        let avatar = Paragraph::new(lines).block(block);
        avatar.render(left, buf);
    }
    fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        let [_, bottom] = vertical![==95%, ==5%]
            .vertical_margin(1)
            .split(area)
            .to_vec()
            .try_into()
            .unwrap();
        let lines = self.footer.iter().map(Line::raw).collect::<Vec<_>>();
        let footer = Paragraph::new(lines).centered().bold().light_cyan().italic();
        footer.render(bottom, buf);
    }
}

impl StatefulWidget for Dashboard<'_> {
    type State = TableState;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        self.render_header(area, buf);
        self.render_table(area, buf, state);
        self.render_avatar(area, buf);
        self.render_footer(area, buf);
    }
}
