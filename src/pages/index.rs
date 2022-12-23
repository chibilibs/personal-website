use web_sys::{window, ScrollToOptions};
use yew::prelude::*;

#[function_component]
pub fn Index() -> Html {
    fn go_top_btn() {
        let mut options = ScrollToOptions::new();
        options.top(0.0);
        window().unwrap().scroll_to_with_scroll_to_options(&options)
    }

    fn on_window_load() {}

    const ASCII: &str = "\
⣿⣿⣿⣿⣯⣿⣿⠄⢠⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡟⠈⣿⣿⣿⣿⣿⣿⣆⠄
⢻⣿⣿⣿⣾⣿⢿⣢⣞⣿⣿⣿⣿⣷⣶⣿⣯⣟⣿⢿⡇⢃⢻⣿⣿⣿⣿⣿⢿⡄
⠄⢿⣿⣯⣏⣿⣿⣿⡟⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣧⣾⢿⣮⣿⣿⣿⣿⣾⣷
⠄⣈⣽⢾⣿⣿⣿⣟⣄⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣝⣯⢿⣿⣿⣿⣿
⣿⠟⣫⢸⣿⢿⣿⣾⣿⢿⣿⣿⢻⣿⣿⣿⢿⣿⣿⣿⢸⣿⣼⣿⣿⣿⣿⣿⣿⣿
⡟⢸⣟⢸⣿⠸⣷⣝⢻⠘⣿⣿⢸⢿⣿⣿⠄⣿⣿⣿⡆⢿⣿⣼⣿⣿⣿⣿⢹⣿
⡇⣿⡿⣿⣿⢟⠛⠛⠿⡢⢻⣿⣾⣞⣿⡏⠖⢸⣿⢣⣷⡸⣇⣿⣿⣿⢼⡿⣿⣿
⣡⢿⡷⣿⣿⣾⣿⣷⣶⣮⣄⣿⣏⣸⣻⣃⠭⠄⠛⠙⠛⠳⠋⣿⣿⣇⠙⣿⢸⣿
⠫⣿⣧⣿⣿⣿⣿⣿⣿⣿⣿⣿⠻⣿⣾⣿⣿⣿⣿⣿⣿⣿⣷⣿⣿⣹⢷⣿⡼⠋
⠄⠸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣦⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡟⣿⣿⣿⠄⠄
⠄⠄⢻⢹⣿⠸⣿⣿⣿⣿⣿⣷⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⣼⣿⣿⣿⣿⡟⠄⠄
⠄⠄⠈⢸⣿⠄⠙⢿⣿⣿⣹⣿⣿⣿⣿⣟⡃⣽⣿⣿⡟⠁⣿⣿⢻⣿⣿⢿⠄⠄
⠄⠄⠄⠘⣿⡄⠄⠄⠙⢿⣿⣿⣾⣿⣷⣿⣿⣿⠟⠁⠄⠄⣿⣿⣾⣿⡟⣿⠄⠄
⠄⠄⠄⠄⢻⡇⠸⣆⠄⠄⠈⠻⣿⡿⠿⠛⠉⠄⠄⠄⠄⢸⣿⣇⣿⣿⢿⣿⠄⠄";

    html! {
        <section onload={|_|on_window_load()}  class="container">
            <div class="content header">
                <div class="ascii">
                    <p class="ascii about_me">{ASCII}</p>
                </div>
                <h1 class="about_me">{"Hey there!"}</h1>
            </div>
            <div class="content about-me">
                <div class="bg-about-me">
                    <h2>
                        <span class="let">{"let "}</span>
                        <span class="about_me">{"about_me "}</span>
                        <span class="equals">{"= "}</span>
                        <span class="r">{"r"}</span>
                        <span class="quotes">{"#''"}</span>
                    </h2>
                    <p class="text">
                        <span class="fields_keyword">{"Name:"}</span>
                        {" Guilherme de Oliveira Menezes"}
                    </p>
                    <p class="text">
                        <span class="fields_keyword">{"Nickname:"}</span>
                        {" thechibbis"}</p>
                    <br/>
                    <p class="text">
                        <span class="fields_keyword">{"Description:"}</span>
                        {" I'm a brazilian Rust developer. I'm currently working on my personal projects and seeking knowledge especially on back-end"}</p>
                    <p class="text">{"and of course increasing my abilities as a coder."}</p>
                    <p class="text">{"Actually, I'd consider myself an intermediary programmer focused on backend with rust"}</p>
                    <h2>
                        <span class="quotes">{"''#"}</span>
                    </h2>
                    <h3 class="r">{"You can see some of my projects in the section below"}</h3>
                </div>
            </div>
            <div class="content github-projects text-github"></div>
            <div class="content github-projects"></div>
            <button id="btn" onclick={|_|go_top_btn()}>{"↑"}</button>
        </section>
    }
}
