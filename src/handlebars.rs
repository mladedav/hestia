use handlebars::{handlebars_helper, Handlebars};
use unicode_segmentation::UnicodeSegmentation;

pub fn customize(handlebars: &mut Handlebars) {
    handlebars_helper!(concat: |x: String, y: String| x + &y);
    handlebars.register_helper("concat", Box::new(concat));
    handlebars_helper!(range: |x: u64| (0..x).collect::<Vec<u64>>());
    handlebars.register_helper("range", Box::new(range));
    handlebars_helper!(sub: |x: u64, y: u64| x - y);
    handlebars.register_helper("sub", Box::new(sub));
    handlebars_helper!(splitnl: |x: String| x.split('\n').collect::<Vec<_>>());
    handlebars.register_helper("splitnl", Box::new(splitnl));
    handlebars_helper!(excerpt: |text: String, length: usize| {
        if length < text.len() {
            format!("{}...", text.graphemes(true).collect::<String>())
        } else {
            text
        }
    });
    handlebars.register_helper("excerpt", Box::new(excerpt));
    handlebars_helper!(split: |s: Option<String>| s.filter(|s| !s.trim().is_empty()).map(|s| s.split(',').map(|s| s.to_string()).collect::<Vec<_>>()).unwrap_or_default());
    handlebars.register_helper("split", Box::new(split));
    handlebars_helper!(eq: |x: String, y: String| x == y);
    handlebars.register_helper("eq", Box::new(eq));
    handlebars
        .register_templates_directory(".html.hbs", "templates")
        .unwrap();
}
