use nu_plugin::MsgPackSerializer;
use nu_plugin_ldap::LdapPlugin;

fn main() {
    tracing_subscriber::fmt::init();
    nu_plugin::serve_plugin(&LdapPlugin::new(), MsgPackSerializer);
}
