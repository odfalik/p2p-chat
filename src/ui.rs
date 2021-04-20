use crate::State;
use druid::widget::*;
use druid::{AppLauncher, Widget, WidgetExt, WindowDesc};

pub fn main() {
    let main_window = WindowDesc::new(build_root_widget)
        .title("Chat")
        .window_size((750.0, 500.0));

    let initial_state = State {
        ip: "".into(),
        port: "4390".into(),
        username: "".into(),
        draft: "".into(),
        msgs: String::from("Messages"),
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<State> {
    // a label that will determine its text based on the current app data.
    // let label = Label::new(|data: &State, _env: &Env| format!("Hello {}", data.draft));

    let ip_box = TextBox::new()
        .with_placeholder("IP Address")
        .expand_width()
        .lens(State::ip);

    let user_box = TextBox::new()
        .with_placeholder("Nickname")
        .expand_width()
        .lens(State::ip);

    let col1 = Flex::column()
        .with_child(ip_box)
        .with_child(user_box)
        .fix_width(200.0);

    let message_list = Label::raw()
        .with_line_break_mode(LineBreaking::Overflow)
        .lens(State::msgs)
        .expand_width();

    let scroll = Scroll::new(message_list).expand_width();

    let draft_box = TextBox::new()
        .with_placeholder("Message")
        .expand_width()
        .lens(State::draft);

    let send_btn = Button::new("Send").on_click(|_ctx, state: &mut State, _env| {
        // TODO 
        state.draft = String::from("");
    });

    let col2 = Flex::column()
        .with_child(scroll)
        .with_spacer(15.0)
        .with_flex_child(draft_box, 5.0)
        .with_flex_child(send_btn, 5.0)
        .must_fill_main_axis(true)
        .fix_width(500.0);

    let row = Flex::row()
        .with_child(col1)
        .with_flex_spacer(15.0)
        .with_child(col2);

    Padding::new(15.0, Align::centered(row))
}
