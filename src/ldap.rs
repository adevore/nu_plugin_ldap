pub struct LdapPlugin {
    pub(crate) main_runtime: tokio::runtime::Runtime,
}

impl LdapPlugin {
    pub fn new() -> Self {
        LdapPlugin {
            main_runtime: tokio::runtime::Runtime::new().unwrap(),
        }
    }
}
