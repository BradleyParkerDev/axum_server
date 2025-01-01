use axum::response::Html;
use minijinja::{Environment, context};
use std::fs;

pub async fn home_page_controller() -> Html<String> {
    // Initialize the MiniJinja environment
    let mut env = Environment::new();

    // Load the `home_page.html` template into the environment
    let template_path = "resources/templates/pages/home_page.html";
    let template_content = fs::read_to_string(template_path)
        .expect("Failed to read template file");
    env.add_template("home_page.html", &template_content)
        .expect("Failed to add template");

    // Render the template with context
    let template = env.get_template("home_page.html")
        .expect("Template not found");
    let rendered = template
        .render(context! {
            greeting => "Hello, World!!! This is an axum server!!!",
        })
        .expect("Failed to render template");

    // Return the rendered HTML as a response
    Html(rendered)
}
