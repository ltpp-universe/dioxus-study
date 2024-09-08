use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
mod button;
mod checkbox;
mod dialog;
mod img;
mod input;
mod input_number;
mod link;
mod radio;
mod select;
mod slider;
mod switch;
mod video;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(app);
}

fn app() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let mut count: Signal<i32> = use_signal(|| 0);

    rsx! {
        div {
            // 新增的 fixed 布局背景元素
            div {
                style: "position: fixed; top: 0; left: 0; width: 100vw; height: 100vh; background-color: lightblue; z-index: -10;",
                h1 { "High-Five counter" }
            },
            // 使用 flex 布局的 div
            div {
                style: "display: flex; justify-content: space-around; align-items: center; height: 100px;",
                h1 { "High-Five counter: {count}" }
                button { onclick: move |_| count += 1, "Up high!" }
                button { onclick: move |_| count -= 1, "Down low!" }
            }

            // 跳转链接
            Link {
                to: Route::Blog {
                    id: count()
                },
                "Go to blog"
            }
        }
    }
}
