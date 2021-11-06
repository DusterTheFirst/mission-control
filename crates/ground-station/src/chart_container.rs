use iced::{Container, Length};
use plotters_iced::{Chart, ChartWidget};

pub fn create_chart<'chart, Message: 'chart>(
    chart: &'chart mut impl Chart<Message>,
) -> Container<'chart, Message> {
    Container::new(
        ChartWidget::new(chart)
            .width(Length::Fill)
            .height(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
}
