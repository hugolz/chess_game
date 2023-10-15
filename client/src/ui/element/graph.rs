const NBR_OF_ELEMENTS: usize = 80;

pub struct Graph {
    id: crate::ui::Id,
    position: crate::ui::Position,
    size: ggez::mint::Point2<crate::ui::Value>,
    style: crate::ui::Style,

    values: std::collections::VecDeque<f64>,
    accept_timer: time::SystemTimeDelay,

    text: Option<GraphText>,

    max: f64,
}

impl Graph {
    pub fn new(
        position: crate::ui::Position,
        size: ggez::mint::Point2<crate::ui::Value>,
        style: crate::ui::Style,
        text: Option<GraphText>,
    ) -> Self {
        Self {
            id: crate::ui::Id::new(),
            position,
            size,
            style,
            values: Default::default(),
            accept_timer: time::SystemTimeDelay::new(100),
            text,
            max: 0.,
        }
    }
    pub fn push(&mut self, v: f64) {
        if let time::DelayState::Running = self.accept_timer.ended() {
            return;
        }
        self.accept_timer.restart();

        self.values.push_back(v);

        let mut max = f64::NEG_INFINITY;
        self.values.iter().for_each(|v| {
            if *v > max {
                max = *v
            }
        });
        max *= 1.50;
        self.max = max;

        while self.values.len() > NBR_OF_ELEMENTS {
            self.values.pop_front();
        }
    }
}

pub struct GraphText {
    anchor: crate::ui::Anchor,
    offset: shared::maths::Vec2,
    size: f64,
    color: crate::render::Color,
    text: fn(f64) -> String,
}

impl super::TElement for Graph {
    fn draw(
        &mut self,
        ctx: &mut ggez::Context,
        back_mesh: &mut ggez::graphics::MeshBuilder,
        ui_mesh: &mut ggez::graphics::MeshBuilder,
        front_mesh: &mut ggez::graphics::MeshBuilder,
        render_request: &mut crate::render::RenderRequest,
    ) -> ggez::GameResult {
        let rect = self.get_computed_rect(ctx);
        let horizontal_space = rect.size().x as f32 / NBR_OF_ELEMENTS as f32;

        // draw background
        if let Some(bg) = self.style.get_bg() {
            back_mesh.rectangle(
                ggez::graphics::DrawMode::fill(),
                self.get_computed_rect(ctx).into(),
                (*bg.get_color()).into(),
            )?;
        }

        // draw border
        if let Some(border) = self.style.get_border() {
            let r = shared::maths::Rect::new(
                rect.r_topleft() - border.get_size() / 2.,
                rect.size() + *border.get_size(),
                rect.rotation(),
            );

            front_mesh.rectangle(
                ggez::graphics::DrawMode::stroke(*border.get_size() as f32),
                r.into(),
                (*border.get_color()).into(),
            )?;
        };

        // draw debug text
        if let Some(graph_text) = &self.text {
            if !self.values.is_empty() {
                let text = (graph_text.text)(*self.values.back().unwrap());
                let mut ggtext = ggez::graphics::Text::new(text);
                ggtext.set_layout(ggez::graphics::TextLayout::top_left());

                let p = graph_text
                    .anchor
                    .compute(rect.size(), ggtext.measure(ctx).unwrap().into())
                    + rect.r_topleft();

                render_request.add(
                    ggtext,
                    crate::render::DrawParam::default()
                        .pos(p)
                        .color(graph_text.color),
                    crate::render::Layer::UiForeground,
                );
            }
        }

        let mut saved_height = None;
        for (i, val) in self.values.iter().enumerate() {
            let curr_height = (*val as f32 / self.max as f32) * rect.size().y as f32;
            if curr_height.is_nan() {
                warn!(
                    "Could not draw Graph id '{}' because the given value is NAN",
                    self.id
                );
                continue;
            }

            // trace!("{curr_height} - {}", self.max);
            if saved_height.is_none() {
                saved_height = Some(curr_height);
                continue;
            }

            ui_mesh.line(
                &[
                    [
                        rect.aa_botleft().x as f32 + horizontal_space * (i - 1) as f32 - 1.,
                        rect.aa_botleft().y as f32 - saved_height.unwrap(),
                    ],
                    [
                        rect.aa_botleft().x as f32 + horizontal_space * i as f32 + 1.,
                        rect.aa_botleft().y as f32 - curr_height,
                    ],
                ],
                3.,
                (*self.style.get_color()).into(),
            )?;

            saved_height = Some(curr_height)
        }

        Ok(())
    }
    fn get_size_value(&self) -> &ggez::mint::Point2<crate::ui::Value> {
        &self.size
    }
    fn get_pos_value(&self) -> &crate::ui::Position {
        &self.position
    }
    fn get_id(&self) -> shared::id::Id {
        self.id
    }
}

impl GraphText {
    pub fn anchor(mut self, new_anchor: crate::ui::Anchor) -> Self {
        self.anchor = new_anchor;
        self
    }

    pub fn offset(mut self, new_offset: impl Into<shared::maths::Vec2>) -> Self {
        self.offset = new_offset.into();
        self
    }
    pub fn size(mut self, new_size: f64) -> Self {
        self.size = new_size;
        self
    }

    pub fn color(mut self, new_color: crate::render::Color) -> Self {
        self.color = new_color;
        self
    }

    pub fn text(mut self, new_text: fn(f64) -> String) -> Self {
        self.text = new_text;
        self
    }
}

impl Default for GraphText {
    fn default() -> Self {
        Self {
            anchor: crate::ui::Anchor::Topleft,
            offset: shared::maths::Vec2::ZERO,
            size: 10.,
            color: crate::render::Color::WHITE,
            text: |val| -> String { format!("{val:.3}") },
        }
    }
}