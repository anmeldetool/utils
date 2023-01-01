use crate::App;

use crate::error::AuthError;

pub trait Plugin {
    fn init(&self, app: &App) -> Result<(), ()>;

    // fn new() -> Self
    // where
    //     Self: Sized;

    fn shutdown(&self, app: &App) -> Result<(), ()>;

    fn clone_boxed(&self) -> Box<dyn Plugin>;
}

pub trait UserPlugin: Plugin {
    fn render_input(&self) -> String; // html/js/css template -> new user form
    fn render_edit(&self, user_id: &str) -> String; // html/js/css template -> change user/profile form
    fn render_display(&self, user_id: &str) -> String;
}

pub trait AuthPlugin: UserPlugin {
    fn authenticate(&self, name: &str, password: &str) -> Result<(), AuthError>;
}

pub trait ProjectPlugin: Plugin {}

pub trait FormPlugin: Plugin {}
