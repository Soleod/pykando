use pykando::context::*;
use pykando::events;

fn main() {
    let context = Context::new();
    let (event_loop, window) = (context.event_loop, context.window);
    events::run(window, event_loop);
}