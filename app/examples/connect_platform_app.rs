use std::sync::Arc;
use std::thread;

use app::abstract_app::AbstractApp;
use app::app::APP;
use app::app_remote_connector::APP_REMOTE_CONNECTOR;
use common::structs::sensor_data::SensorData;

struct AppConnectDemo {
    abstract_app: AbstractApp,
}

impl AppConnectDemo {
    pub fn new() -> Self {
        let abstract_app = AbstractApp::new();
        let mut app = Self { abstract_app };
        app.config_app();
        app
    }

    pub fn new_and_log_set() -> Self {
        let app = Self::new();
        app.log_set();
        app
    }
}

impl APP for AppConnectDemo {
    fn get_msg(&self, sensor_name: String, value: SensorData) {
        todo!()
    }

    fn config_app(&mut self) {
        self.abstract_app
            .set_app_name(Arc::new(String::from("app_connect_demo")));
        self.abstract_app
            .set_app_description(Arc::new(String::from("this is app_connect_demo")));
    }

    fn super_reference(&self) -> &AbstractApp {
        &self.abstract_app
    }
}

fn main() {
    let app = Arc::new(AppConnectDemo::new_and_log_set());
    APP_REMOTE_CONNECTOR
        .connect_platform(String::from("127.0.0.1"), 9090)
        .unwrap();
    APP_REMOTE_CONNECTOR.check_connected().unwrap();
    APP_REMOTE_CONNECTOR.register_app(app.clone()).unwrap();
    APP_REMOTE_CONNECTOR.unregister_app(app.clone()).unwrap();
    APP_REMOTE_CONNECTOR.disconnect_platform().unwrap();
    thread::sleep(std::time::Duration::from_secs(1));
}
