use crate::{AuthPlugin, FormPlugin, ProjectPlugin, UserPlugin};

pub struct App {}

impl App {
    pub fn new() -> Self {
        App {}
    }

    pub fn register_user_plugin<P: UserPlugin>(&self, _plugin: &P) -> Result<(), ()> {
        // TODO: check if plugin has permissions to register as user plugin
        todo!()
    }

    pub fn register_auth_plugin<P: AuthPlugin>(&self, _plugin: &P) -> Result<(), ()> {
        // TODO: check if plugin has permissions to register as auth plugin
        todo!()
    }

    pub fn register_project_plugin<P: ProjectPlugin>(&self, _plugin: &P) -> Result<(), ()> {
        // TODO: check if plugin has permissions to register as project plugin
        todo!()
    }
    pub fn register_form_plugin<P: FormPlugin>(&self, _plugin: &P) -> Result<(), ()> {
        // TODO: check if plugin has permissions to register as form plugin
        todo!()
    }
}
