mod server;

use std::sync::Arc;
use crossbeam_channel::unbounded;
use druid::widget::*;
use druid::{AppLauncher, Data, Env, Lens, Widget, WidgetExt, WindowDesc};
use std::sync::RwLock;

#[derive(Clone, Data, Lens, Debug)]
struct State {
    ip: String,
    port: String,
    username: String,
    draft: String,
    messages: String,
}

fn main() {
    let main_window = WindowDesc::new(build_root_widget)
        .title("Chat")
        .window_size((750.0, 500.0));

    let initial_state = State {
        ip: "".into(),
        port: "4390".into(),
        username: "".into(),
        draft: "".into(),
        messages: "".into(),
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<State> {
    
    let user_box = TextBox::new()
        .with_placeholder("Nickname")
        .expand_width()
        .padding(5.0)
        .lens(State::username);

    let ip_box = TextBox::new()
        .with_placeholder("IP Address")
        .expand_width()
        .padding(5.0)
        .lens(State::ip);

    let connect_btn = Button::new(|data: &State, _env: &Env| {
        format!(
            "Connect {}",
            if data.ip.len() > 0 {
                String::from("to ") + &data.ip
            } else {
                String::from("")
            }
        )
    })
    .padding(15.0)
    .on_click(|_ctx, _t, _env| println!("Connecting"));

    let messages = Arc::new(RwLock::new(String::new()));

    let launch_server_btn =
        Button::new("Start Server")
            .padding(15.0)
            .on_click(move |_ctx, data: &mut State, _env| {
                data.ip = String::from(""); // testing modifying state on cb
                let (s, r) = unbounded();
                // let _res: Vec<String> = r.try_iter().collect();
                // println!("recv {}", _res.len());
                server::start(s, r, messages.clone());
            });

    let col1 = Flex::column()
        .with_child(user_box)
        .with_child(ip_box)
        .with_child(connect_btn)
        .with_child(launch_server_btn);

    let messages_label = Label::new(move |_data: &State, _env: &Env| {
        println!("messages read");
        *(messages.read().unwrap())
    });
    
    let scroll = Scroll::new(messages_label);

    let draft_box = TextBox::new()
        .with_placeholder("Message")
        .expand_width()
        .lens(State::draft);

    let col2 = Flex::column()
        .with_child(scroll)
        .with_spacer(15.0)
        .with_flex_child(draft_box, 5.0)
        .must_fill_main_axis(true);

    let row = Flex::row()
        .with_flex_child(col1, 250.0)
        .with_flex_spacer(15.0)
        .with_flex_child(col2, 500.0);

    Padding::new(15.0, Align::centered(row))
}
