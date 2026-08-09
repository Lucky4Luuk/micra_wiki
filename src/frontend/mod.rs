//! This module handles everything the frontend needs to display a nice page.
//! It uses Handlebars for templating.

use handlebars::Handlebars;

pub mod wiki;

pub struct PageTemplater {
    hb: Handlebars<'static>,
}

impl PageTemplater {
    pub fn new() -> Self {
        let mut hb = Handlebars::new();

        hb.register_template_file("base", "templates/base.hbs")
            .expect("Failed to create a template from `templates/base.hbs`!");

        Self { hb }
    }
}
