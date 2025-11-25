use nu_plugin::MsgPackSerializer;
use nu_plugin_ldap::LdapPlugin;

fn main() {
    nu_plugin::serve_plugin(&LdapPlugin::new(), MsgPackSerializer);
}
