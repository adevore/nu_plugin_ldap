use nu_plugin::EvaluatedCall;
use nu_protocol::LabeledError;

pub(crate) struct Opts {
    pub uri: Option<String>,
    pub scope: Option<String>,
    pub filter: String,
    pub attributes: Vec<String>,
    pub size_limit: Option<i32>,
    pub time_limit: Option<i32>,
    pub deref_aliases: Option<i32>,
    pub extensions: Option<String>,
    pub controls: Option<String>,
    pub binddn: Option<String>,
    pub bindpw: Option<String>,
    pub basedn: Option<String>,
    pub typesonly: bool,
}

pub(crate) fn parse_opts(call: &EvaluatedCall) -> Result<Opts, LabeledError> {
    let uri = call.get_flag::<String>("uri")?;
    let scope = call.get_flag("scope")?;
    let filter = call.req(1)?;
    let attributes = call.rest(2)?;
    let size_limit = call.get_flag("size-limit")?;
    let time_limit = call.get_flag("time-limit")?;
    let deref_aliases = call.get_flag("deref-aliases")?;
    let extensions = call.get_flag("extensions")?;
    let controls = call.get_flag("controls")?;
    let binddn = call.get_flag("binddn")?;
    let bindpw = call.get_flag("password")?;
    //let bindpw_prompt: bool = call.get_flag("password-prompt")?.unwrap_or(false);
    let basedn = call.get_flag("basedn")?;
    let typesonly = call.get_flag("typesonly")?.unwrap_or(false);
    Ok(Opts {
        uri,
        scope,
        filter,
        attributes,
        size_limit,
        time_limit,
        deref_aliases,
        extensions,
        controls,
        binddn,
        bindpw,
        basedn,
        typesonly,
    })
}
