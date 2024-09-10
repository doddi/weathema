use anathema::backend::tui::Style;
use anathema::component::{Color, Component, ComponentId, Elements, Emitter, List, State, Value};
use anathema::default_widgets::Canvas;
use anathema::geometry::LocalPos;
use anathema::prelude::*;
use anathema::runtime::RuntimeBuilder;

#[derive(Debug, Default)]
struct GraphComponent;

impl GraphComponent {
    #[tracing::instrument]
    fn populate_graph(
        &mut self,
        canvas: &mut Canvas,
        point_width: u16,
        data_points: &[u16],
        min: &u16,
        max: &u16,
        style: &Style,
    ) {
        for (pt_idx, value) in data_points.iter().enumerate() {
            // let y = (max - min) - (*value - min);
            let y = (max - min) - (*value - min);
            self.add_to_canvas(point_width, pt_idx, y, canvas, style);
        }
    }

    #[tracing::instrument]
    fn add_to_canvas(
        &self,
        point_width: u16,
        pt_idx: usize,
        y: u16,
        canvas: &mut Canvas,
        style: &Style,
    ) {
        for width_idx in 0..point_width {
            let x = (pt_idx as u16 * point_width) + width_idx;
            canvas.put('*', *style, LocalPos::new(x, y));
        }
    }
}

impl Component for GraphComponent {
    type State = GraphComponentState;
    type Message = GraphComponentMessage;

    fn message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut elements: Elements<'_, '_>,
        _context: Context<'_, Self::State>,
    ) {
        // Find the range of the data points
        let min = message.max_temp_points.iter().min().unwrap();
        let max = message.max_temp_points.iter().max().unwrap();

        let range = if max - min < 10 { 10 } else { max - min };
        state.height.set(range);

        let point_width = state.point_width.to_ref();
        let width = if (message.max_temp_points.len()) < 10 {
            10
        } else {
            message.max_temp_points.len() as u16
        } * *point_width;
        state.width.set(width);

        // Populate max temps in the forecast
        let mut style = Style::new();
        style.set_fg(Color::Red);
        elements.by_tag("canvas").first(|el, _| {
            let canvas = el.to::<Canvas>();
            self.populate_graph(
                canvas,
                *point_width,
                &message.max_temp_points,
                min,
                max,
                &style,
            );
        });
    }
}

#[derive(Debug, State)]
struct GraphComponentState {
    title: Value<String>,

    point_width: Value<u16>,

    height: Value<u16>,
    width: Value<u16>,

    data_points: Value<List<u8>>,
}

impl GraphComponentState {
    fn new() -> Self {
        let data_points = List::from_iter(vec![]);
        Self {
            title: Value::new("Graph".to_string()),

            point_width: Value::new(3),
            height: Value::new(10),
            width: Value::new(10),
            data_points,
        }
    }
}

pub struct GraphComponentMessage {
    max_temp_points: Vec<u16>,
}

pub fn create_component(
    runtime: &mut RuntimeBuilder<TuiBackend, impl GlobalEvents>,
) -> ComponentId<GraphComponentMessage> {
    runtime
        .register_component(
            "graphComponent",
            "src/templates/graph_component.aml",
            GraphComponent,
            GraphComponentState::new(),
        )
        .unwrap()
}

pub(crate) fn update_component(
    emitter: &Emitter,
    id: ComponentId<GraphComponentMessage>,
    max_temp_points: Vec<u16>,
) {
    let _ = emitter.emit(id, GraphComponentMessage { max_temp_points });
}
