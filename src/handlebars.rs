use handlebars::{Handlebars, handlebars_helper};
use unicode_segmentation::UnicodeSegmentation;

pub fn handlebars() -> Handlebars<'static> {
    let mut handlebars = Handlebars::new();

    handlebars_helper!(range: |x: u64| (0..x).collect::<Vec<u64>>());
    handlebars.register_helper("range", Box::new(range));
    handlebars_helper!(sub: |x: u64, y: u64| x - y);
    handlebars.register_helper("sub", Box::new(sub));
    handlebars_helper!(excerpt: |text: String, length: usize| {
        if length < text.len() {
            format!("{}...", text.graphemes(true).collect::<String>())
        } else {
            text
        }
    });
    handlebars.register_helper("excerpt", Box::new(excerpt));
    handlebars.register_templates_directory(".html.hbs", "templates").unwrap();
    handlebars
}
