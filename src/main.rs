fn main() {
    dioxus_desktop::launch_cfg(
        coven::App,
        dioxus_desktop::Config::new(), // .with_custom_head(r#"<link rel="stylesheet" href="styles.css">"#.to_string()),
    )
}
