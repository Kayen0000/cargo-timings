use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    text::Text,
    widgets::{List, ListItem, Widget},
};

pub struct WrappingColumnList<'a> {
    items: Vec<ListItem<'a>>,
    column_width: u16,
}

impl<'a> WrappingColumnList<'a> {
    pub fn new<T>(items: Vec<T>, column_width: u16) -> Self
    where
        T: Into<Text<'a>>,
    {
        let items = items.into_iter().map(|t| ListItem::from(t)).collect();
        WrappingColumnList { items, column_width }
    }
}

impl<'a> Widget for WrappingColumnList<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.items.is_empty() || area.height == 0 || area.width == 0 {
            return;
        }

        let items_per_column = area.height as usize;

        let columns_data: Vec<&[ListItem]> = self.items.chunks(items_per_column).collect();
        let total_columns = columns_data.len();

        let mut constraints = vec![Constraint::Length(self.column_width); total_columns];

        let max_visible_columns = (area.width / self.column_width) as usize;
        constraints.truncate(max_visible_columns);

        let horizontal_chunks = Layout::default().direction(Direction::Horizontal).constraints(constraints).split(area);

        for (i, &col_items) in columns_data.iter().enumerate().take(max_visible_columns) {
            let list_widget = List::new(col_items.to_vec());
            list_widget.render(horizontal_chunks[i], buf);
        }
    }
}
