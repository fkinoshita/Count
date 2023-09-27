// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::HashMap;

use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};
use gsv;

use gettextrs::gettext;

use crate::application::Application;
use crate::config::{APP_ID, PROFILE, VERSION};

use crate::calc::{lexer::Lexer, parser::{Parser, Atom, Expression}, interpreter::Interpreter};

mod imp {
    use super::*;

    #[derive(Debug, gtk::CompositeTemplate)]
    #[template(resource = "/com/felipekinoshita/Count/ui/window.ui")]
    pub struct Window {
        pub settings: gio::Settings,
        pub context: HashMap<String, Expression>,

        #[template_child]
        pub text_view: TemplateChild<gsv::View>,
        #[template_child]
        pub text_buffer: TemplateChild<gsv::Buffer>,
    }

    impl Default for Window {
        fn default() -> Self {
            Self {
                settings: gio::Settings::new(APP_ID),
                context: HashMap::new(),

                text_view: TemplateChild::default(),
                text_buffer: TemplateChild::default(),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "Window";
        type Type = super::Window;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();

            klass.install_action("win.about", None, move |obj, _, _| {
                obj.show_about_dialog();
            });
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Window {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            // Devel profile
            if PROFILE == "Devel" {
                obj.add_css_class("devel");
            }
        }
    }

    #[gtk::template_callbacks]
    impl Window {
        #[template_callback]
        fn on_text_changed(&self, text_buffer: gsv::Buffer) {
            let input = text_buffer.text(&text_buffer.start_iter(), &text_buffer.end_iter(), true);
            let lines: Vec<_> = input.split("\n").collect();

            let mut context = self.obj().imp().context.clone();

            for line in lines.iter() {
                let value = self.obj().evaluate(line.to_string(), &mut context);
                println!("{value}");
            }
        }
    }

    impl WidgetImpl for Window {}
    impl WindowImpl for Window {}

    impl ApplicationWindowImpl for Window {}
    impl AdwApplicationWindowImpl for Window {}
}

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Root;
}

impl Window {
    pub fn new(application: &Application) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    fn evaluate(&self, input: String, context: &mut HashMap<String, Expression>) -> String {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        let mut parser = Parser::new(tokens);
        let expressions = parser.parse();

        let mut interpreter = Interpreter::new();

        let mut return_value = String::new();

        for expr in expressions {
            let evaluated = interpreter.evaluate(expr, context);
            let value = interpreter.literal_value(evaluated);

            let value = match value {
                Some(Atom::Boolean(boolean)) => {
                    boolean.to_string()
                },
                Some(Atom::Number(number)) => {
                    number.to_string()
                },
                Some(Atom::Name(name)) => {
                    let e = interpreter.evaluate(Expression::Literal(Atom::Name(name.clone())), context);
                    let v = interpreter.literal_value(e);

                    let x = match v {
                        Some(Atom::Boolean(boolean)) => format!("{boolean}").to_string(),
                        Some(Atom::Number(number)) => format!("{number}").to_string(),
                        _ => "".to_string(),
                    };

                    x.to_string()
                },
                None => "".to_string(),
            };

            return_value = value;
        }

        return_value
    }

    fn show_about_dialog(&self) {
        let dialog = adw::AboutWindow::builder()
            .application_icon(APP_ID)
            .application_name(gettext("Count"))
            .license_type(gtk::License::Gpl30)
            .comments(gettext("Math made easy"))
            .website("https://github.com/fkinoshita/Count")
            .issue_url("https://github.com/fkinoshita/Count/issues/new")
            .version(VERSION)
            .transient_for(self)
            .translator_credits(gettext("translator-credits"))
            .developer_name("Felipe Kinoshita")
            .developers(vec!["Felipe Kinoshita <fkinoshita@gnome.org>"])
            .copyright("© 2023 Felipe Kinoshita.")
            .release_notes(gettext("
                <p></p>"
            ))
            .build();

        dialog.present();
    }
}
