fn main() {
    let (style_sheet, _) = turf::inline_style_sheet!("src/styles/theme.scss");

    dioxus_desktop::launch_cfg(
        coven::App,
        dioxus_desktop::Config::new()
            .with_custom_head(r#"<link rel="stylesheet" href="src/styles.css">"#.to_string()),
    )
}
