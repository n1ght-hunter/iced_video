//! A widget that overlays another widget with a modal.
//! this is used to overlay the video controls on top of the video.

use iced::{
    advanced::{
        layout, overlay, renderer,
        widget::{self, Tree},
        Clipboard, Layout, Shell, Widget,
    }, border::Radius, event, mouse, Alignment, Color, Element, Event, Length, Point, Rectangle, Size, Vector
};

/// A widget that overlays another widget with a modal.
#[allow(missing_debug_implementations)]
pub struct Overlay<'a, Message,Theme, Renderer> {
    base: Element<'a, Message,Theme, Renderer>,
    modal: Element<'a, Message,Theme, Renderer>,
}

impl<'a, Message,Theme, Renderer> Overlay<'a, Message,Theme, Renderer> {
    /// Returns a new [`Modal`]
    pub fn new(
        base: impl Into<Element<'a, Message,Theme, Renderer>>,
        modal: impl Into<Element<'a, Message,Theme, Renderer>>,
    ) -> Self {
        Self {
            base: base.into(),
            modal: modal.into(),
        }
    }
}

impl<'a, Message,Theme, Renderer> Widget<Message,Theme, Renderer> for Overlay<'a, Message,Theme, Renderer>
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

    fn size(&self) -> Size<Length> {
        self.base.as_widget().size()
    }

    fn layout(&self, tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,) -> layout::Node {
        self.base.as_widget().layout(tree, renderer, limits)
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor_position: iced::mouse::Cursor,
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
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: iced::mouse::Cursor,
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
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message,Theme, Renderer>> {
        Some(overlay::Element::new(
            Box::new(OverlayInternal {
                position: layout.bounds().position() + translation,
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
        cursor_position: iced::mouse::Cursor,
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

struct OverlayInternal<'a, 'b, Message,Theme, Renderer> {
    content: &'b mut Element<'a, Message,Theme, Renderer>,
    tree: &'b mut Tree,
    size: Size,
    position: Point,
}

impl<'a, 'b, Message,Theme, Renderer> overlay::Overlay<Message,Theme, Renderer>
    for OverlayInternal<'a, 'b, Message,Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
    Message: Clone,
{
    fn layout(&mut self, renderer: &Renderer, _bounds: Size) -> layout::Node {
        let limits = layout::Limits::new(Size::ZERO, self.size)
            .width(Length::Fill)
            .height(Length::Fill);

        let child = self.content.as_widget().layout(&mut self.tree, renderer, &limits).align(Alignment::Center, Alignment::Center, limits.max());

         layout::Node::with_children(self.size, vec![child]).move_to(self.position)

  
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: iced::mouse::Cursor,
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
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: iced::mouse::Cursor,
    ) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: iced::Border { 
                    radius: Radius::from(0.0),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: iced::Shadow::default(),
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
        cursor_position: iced::mouse::Cursor,
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

impl<'a, Message,Theme, Renderer> From<Overlay<'a,Message,Theme, Renderer>> for Element<'a, Message,Theme, Renderer>
where
    Renderer: 'a + iced::advanced::Renderer,
    Theme: 'a ,
    Message: 'a + Clone,
{
    fn from(modal: Overlay<'a, Message,Theme, Renderer>) -> Self {
        Element::new(modal)
    }
}
