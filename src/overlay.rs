//! A widget that overlays another widget with a modal.
//! this is used to overlay the video controls on top of the video.

use iced::{Element, advanced::{Widget, widget::{Tree, self}, Layout, Clipboard, Shell, layout, renderer, overlay, mouse}, Length, Event, Point, event, Rectangle, Size, Alignment, BorderRadius, Color};

/// A widget that overlays another widget with a modal.
#[allow(missing_debug_implementations)]
pub struct Overlay<'a, Message, Renderer> {
    base: Element<'a, Message, Renderer>,
    modal: Element<'a, Message, Renderer>,
}

impl<'a, Message, Renderer> Overlay<'a, Message, Renderer> {
    /// Returns a new [`Modal`]
    pub fn new(
        base: impl Into<Element<'a, Message, Renderer>>,
        modal: impl Into<Element<'a, Message, Renderer>>,
    ) -> Self {
        Self {
            base: base.into(),
            modal: modal.into(),
        }
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Overlay<'a, Message, Renderer>
where
    Renderer: iced::advanced::Renderer,
    Message: Clone,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.base), Tree::new(&self.modal)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.base, &self.modal]);
    }

    fn width(&self) -> Length {
        self.base.as_widget().width()
    }

    fn height(&self) -> Length {
        self.base.as_widget().height()
    }

    fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        self.base.as_widget().layout(renderer, limits)
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor_position: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        self.base.as_widget_mut().on_event(
            &mut state.children[0],
            event,
            layout,
            cursor_position,
            renderer,
            clipboard,
            shell,
            viewport,
        )
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &<Renderer as iced::advanced::Renderer>::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.base.as_widget().draw(
            &state.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor_position,
            viewport,
        );
    }

    fn overlay<'b>(
        &'b mut self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        _renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        Some(overlay::Element::new(
            layout.position(),
            Box::new(OverlayInternal {
                content: &mut self.modal,
                tree: &mut state.children[1],
                size: layout.bounds().size(),
            }),
        ))
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        cursor_position: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.base.as_widget().mouse_interaction(
            &state.children[0],
            layout,
            cursor_position,
            viewport,
            renderer,
        )
    }

    fn operate(
        &self,
        state: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation<Message>,
    ) {
        self.base
            .as_widget()
            .operate(&mut state.children[0], layout, renderer, operation);
    }
}

struct OverlayInternal<'a, 'b, Message, Renderer> {
    content: &'b mut Element<'a, Message, Renderer>,
    tree: &'b mut Tree,
    size: Size,
}

impl<'a, 'b, Message, Renderer> overlay::Overlay<Message, Renderer>
    for OverlayInternal<'a, 'b, Message, Renderer>
where
    Renderer: iced::advanced::Renderer,
    Message: Clone,
{
    fn layout(&self, renderer: &Renderer, _bounds: Size, position: Point) -> layout::Node {
        let limits = layout::Limits::new(Size::ZERO, self.size)
            .width(Length::Fill)
            .height(Length::Fill);

        let mut child = self.content.as_widget().layout(renderer, &limits);
        child.align(Alignment::Center, Alignment::Center, limits.max());

        let mut node = layout::Node::with_children(self.size, vec![child]);
        node.move_to(position);

        node
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,

    ) -> event::Status {
        let _content_bounds = layout.children().next().unwrap().bounds();

        let viewport = layout.bounds();

        self.content.as_widget_mut().on_event(
            self.tree,
            event,
            layout.children().next().unwrap(),
            cursor_position,
            renderer,
            clipboard,
            shell,
            &viewport,
        )
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: mouse::Cursor,
    ) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border_radius: BorderRadius::from(0.0),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
            Color {
                a: 0.80,
                ..Color::BLACK
            },
        );

        self.content.as_widget().draw(
            self.tree,
            renderer,
            theme,
            style,
            layout.children().next().unwrap(),
            cursor_position,
            &layout.bounds(),
        );
    }

    fn operate(
        &mut self,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation<Message>,
    ) {
        self.content.as_widget().operate(
            self.tree,
            layout.children().next().unwrap(),
            renderer,
            operation,
        );
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor_position: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            self.tree,
            layout.children().next().unwrap(),
            cursor_position,
            viewport,
            renderer,
        )
    }
}

impl<'a, Message, Renderer> From<Overlay<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: 'a + iced::advanced::Renderer,
    Message: 'a + Clone,
{
    fn from(modal: Overlay<'a, Message, Renderer>) -> Self {
        Element::new(modal)
    }
}
