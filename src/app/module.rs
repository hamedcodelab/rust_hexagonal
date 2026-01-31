use crate::app::app::App;
use crate::user::module::UserModule;
use crate::user::repository::init::UserR;


impl App {
    pub async fn init_user_domain(&mut self) {
        let pool = self.pool.as_ref().expect("pool not initialized").clone();
        let user_repo = UserR::new(pool);
        let user_module = UserModule::new(user_repo);
        self.user = Some(user_module);
    }
}