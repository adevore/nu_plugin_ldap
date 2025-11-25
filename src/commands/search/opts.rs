use std::time::Duration;

use ldap3::DerefAliases;
use nu_plugin::EvaluatedCall;
use nu_protocol::LabeledError;
use url::Url;

pub(crate) struct ConnectOpts {
    pub uri: Url,
    pub binddn: Option<String>,
    pub bindpw: Option<String>,
    pub starttls: bool,
    pub connect_timeout: Option<Duration>,
}

pub(crate) struct SearchOpts {
    pub scope: ldap3::Scope,
    pub filter: String,
    pub attributes: Vec<String>,
    pub size_limit: Option<i32>,
    pub time_limit: Option<i32>,
    pub deref_aliases: DerefAliases,
    pub extensions: Option<String>,
    pub controls: Option<String>,
    pub basedn: String,
    pub typesonly: bool,
}

pub(crate) struct Opts {
    pub connect: ConnectOpts,
    pub search: SearchOpts,
}

pub(crate) fn extract_opts(call: &EvaluatedCall) -> Result<Opts, LabeledError> {
    let uri: Url = call
        .get_flag::<String>("uri")?
        .unwrap_or_else(|| "ldap://localhost:389".to_owned())
        .parse()
        .map_err(|err| LabeledError::new(format!("Invalid URI {err}")))?;
    let scope = match call
        .get_flag::<String>("scope")?
        .as_ref()
        .map(|s| s.as_str())
    {
        Some("base") => ldap3::Scope::Base,
        Some("one") => ldap3::Scope::OneLevel,
        Some("sub") | None => ldap3::Scope::Subtree,
        // Not supported by the ldap3 crate
        //Some("children") => ldap3::Scope::Children,
        Some(scope) => return Err(LabeledError::new(format!("Unknown scope {}", scope))),
    };
    let size_limit = call.get_flag("size-limit")?;
    let time_limit = call.get_flag("time-limit")?;
    let deref_aliases = match call
        .get_flag::<String>("deref-aliases")?
        .as_ref()
        .map(|s| s.as_str())
    {
        Some("never") | None => DerefAliases::Never,
        Some("always") => DerefAliases::Always,
        Some("search") => DerefAliases::Searching,
        Some("find") => DerefAliases::Finding,
        Some(value) => {
            return Err(LabeledError::new(format!(
                "Invalid value for --deref-aliases {}",
                value
            )));
        }
    };
    let extensions = call.get_flag("extensions")?;
    let controls = call.get_flag("controls")?;
    let binddn = call.get_flag("binddn")?;
    let bindpw = call.get_flag("password")?;
    let starttls = call.get_flag("starttls")?.unwrap_or(false);
    let connect_timeout = call.get_flag("connect-timeout")?;
    let basedn = call.get_flag("basedn")?.unwrap_or_default();
    let typesonly = call.get_flag("typesonly")?.unwrap_or(false);
    let filter = call.req(1)?;
    let attributes = call.rest(2)?;
    let connect_opts = ConnectOpts {
        uri,
        binddn,
        bindpw,
        starttls,
        connect_timeout,
    };
    let search_opts = SearchOpts {
        scope,
        filter,
        attributes,
        size_limit,
        time_limit,
        deref_aliases,
        extensions,
        controls,
        basedn,
        typesonly,
    };
    Ok(Opts {
        connect: connect_opts,
        search: search_opts,
    })
}
