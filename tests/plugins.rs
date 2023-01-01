#[test]
fn plugins() {
    use utils::{App, Plugin, UserPlugin};

    struct MyPlugin;
    impl MyPlugin {
        fn new() -> Self {
            MyPlugin
        }
    }
    impl Plugin for MyPlugin {
        fn init(&self, app: &App) -> Result<(), ()> {
            app.register_user_plugin(self).unwrap();
            Ok(())
        }

        fn shutdown(&self, _app: &App) -> Result<(), ()> {
            todo!()
        }

        fn clone_boxed(&self) -> Box<dyn Plugin> {
            Box::new(MyPlugin::new())
        }
    }
    impl UserPlugin for MyPlugin {
        fn render_input(&self) -> String {
            todo!()
        }
        fn render_edit(&self, _: &str) -> String {
            todo!()
        }
        fn render_display(&self, _: &str) -> String {
            todo!()
        }
    }

    let app = App::new();
    let plugins: Vec<Box<dyn Plugin>> = vec![Box::new(MyPlugin::new())];

    plugins.iter().for_each(|x| x.init(&app).unwrap());
}
