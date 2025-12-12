use nu_plugin::{Plugin, PluginCommand};

mod commands;
mod config;
mod ldap;

use commands::*;
pub use ldap::LdapPlugin;

impl Plugin for LdapPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![Box::new(Search), Box::new(Table)]
    }
}
