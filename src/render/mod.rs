use tcod::console::{ Console, Root, BackgroundFlag, FontLayout, FontType };
use tcod::input::{ Key };
use crate::util::{ Point, Window };
use crate::traits::RendererComponent;

pub struct TcodRender {
    console: Root
}

impl RendererComponent for TcodRender {
    fn new(window: &Window) -> TcodRender {
        let console: Root = Root::initializer()
            .font("arial10x10.png", FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(window.width, window.height)
            .title("Carakistan")
            .init();
        TcodRender { console }
    }

    fn wait_for_keypress(&mut self) -> Key {
        self.console.wait_for_keypress(true)
    }

    fn add_object(&mut self, position: Point, symbol: char) {
        self.console.put_char(
            position.x,
            position.y,
            symbol,
            BackgroundFlag::Set
        );
    }

    fn clear(&mut self) {
        self.console.clear();
    }

    fn flush(&mut self) {
        self.console.flush();
    }

    fn window_closed(&mut self) -> bool {
        self.console.window_closed()
    }
}