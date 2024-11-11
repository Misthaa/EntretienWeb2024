#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{Level, info};

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
    launch(App);
}


fn App() -> Element {
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
    let mut count = use_signal(|| 0);

    rsx! {
        div {
            style: "position:sticky; background-color: #D9D9D9; color:black; padding: 20px; display: flex; justify-content: center; gap: 20px; align-items: center; width: 30%; height: 4vh; margin: 0 auto; border-radius: 15px; )",
            h1 { "Rusty"}
            button { "Pedago"}
            button { "Issue"}
            button { "Vox"}
            button { "External"}
            button { }
        }
        div {
            style: "background-color: #D9D9D9; padding: 20px; justify-content: center; align-items: center; gap: 20px; border-radius: 15px; margin: 0 auto; width: 30%; display: flex; margin-top: 20px;",
            class: "form-container",
            h1 { "Form" }
            form {
                label {
                    "First Name"
                }
                input {
                    r#type: "text",
                    id: "name",
                    name: "name",
                    required: "true"
                }

                label {
                    "Name"
                }
                input {
                    r#type: "text",
                    id: "name",
                    name: "name",
                    required: "true"
                }
                label {
                    "Email"
                }
                input {
                    r#type: "email",
                    id: "email",
                    name: "email",
                    required: "true"
                }

                label {
                    "Date"
                }
                input {
                    r#type: "date",
                    id: "date",
                    name: "date",
                    required: "true"
                }

                button {
                    r#type: "submit",
                    "Send"
                }
            }
        }
        div {
            div {
                style: "background-color: #D9D9D9; border-radius: 15px; padding: 20px; width: 400px;",
                h2 { style: "font-size: 1.5rem; margin-bottom: 20px;", "News" },
                div {
                    style: "background-color: #ffffff; border-radius: 10px; padding: 15px; margin-bottom: 20px;",
                    div {
                        style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 10px;",
                        h3 { style: "font-size: 1.2rem; margin: 0;", "Lorem Ipsum" },
                        span { style: "font-size: 0.9rem; color: #888;", "11/11/2024" }
                    }
                    p {
                        style: "font-size: 0.95rem; margin-bottom: 10px;",
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas suscipit ligula pharetra quam venenatis aliquam. Praesent quis elit ac orci tempus ultricies sed quis tellus."
                    }
                    div {
                        style: "color: #f26337; cursor: pointer; display: inline-flex; align-items: center; text-decoration: none;",
                        button {
                            "Read more", span { " →" }
                        }
                    }
                }
                div {
                    style: "background-color: #ffffff; border-radius: 10px; padding: 15px; margin-bottom: 20px;",
                    div {
                        style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 10px;",
                        h3 { style: "font-size: 1.2rem; margin: 0;", "Lorem Ipsum" },
                        span { style: "font-size: 0.9rem; color: #888;", "20/08/2024" }
                    }
                    p {
                        style: "font-size: 0.95rem; margin-bottom: 10px;",
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas suscipit ligula pharetra quam venenatis aliquam. Praesent quis elit ac orci tempus ultricies sed quis tellus."
                    }
                    div {
                        style: "color: #f26337; cursor: pointer; display: inline-flex; align-items: center; text-decoration: none;",
                        button {
                            "Read more", span { " →" }
                        }
                    }
                }
            }
        }
    }
}

