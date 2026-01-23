use ldap3::{Ldap, LdapConnAsync, LdapConnSettings, SearchEntry, SearchStream};
use nu_plugin::{EngineInterface, EvaluatedCall};
use nu_protocol::{
    LabeledError, ListStream, PipelineData, Record, ShellError, Signals, Span, Value,
};

use tracing::instrument;
use tracing::{debug, error, info, trace, warn};

use super::opts::{ConnectOpts, SearchOpts, extract_opts};
use crate::LdapPlugin;

#[instrument]
pub(crate) async fn search_impl(
    plugin: &LdapPlugin,
    engine: &EngineInterface,
    call: &EvaluatedCall,
    input: PipelineData,
) -> Result<PipelineData, LabeledError> {
    let span = call.head;
    let opts = extract_opts(call)?;
    let mut ldap = ldap_client(plugin, &opts.connect).await?;

    let (tx, mut rx) = tokio::sync::mpsc::channel::<Result<Value, LabeledError>>(100usize);
    plugin.spawn(async move {
        let mut search_stream = query_impl(&mut ldap, &opts.search, span).await?;
        loop {
            match search_stream.next().await {
                Ok(Some(result)) => {
                    let entry = SearchEntry::construct(result);
                    let record = shape_entry(&entry)?;
                    tx.send(Ok(record)).await.unwrap();
                }
                Ok(None) => break,
                Err(err) => {
                    tx.send(Err(LabeledError::new(format!(
                        "Error fetching search results: {}",
                        err
                    ))))
                    .await
                    .unwrap();
                    break;
                }
            }
        }
        Ok::<(), LabeledError>(())
    });

    Ok(PipelineData::ListStream(
        ListStream::new(
            std::iter::from_fn(move || {
                tokio::task::block_in_place(|| {
                    rx.blocking_recv()
                        .map(|resp| resp.unwrap_or_else(|err| Value::error(err.into(), span)))
                })
            }),
            Span::unknown(),
            // TODO: Handle signals properly
            Signals::empty(),
        ),
        None,
    ))
}
#[instrument]
async fn query_impl<'a>(
    ldap: &'a mut Ldap,
    opts: &'a SearchOpts,
    span: Span,
) -> Result<SearchStream<'a, String, &'a Vec<String>>, LabeledError> {
    info!("Starting query");
    let mut ldap_search_options = ldap3::SearchOptions::new()
        .deref(opts.deref_aliases)
        .typesonly(opts.typesonly);
    if let Some(time_limit) = opts.time_limit {
        ldap_search_options = ldap_search_options.timelimit(time_limit);
    }
    if let Some(size_limit) = opts.size_limit {
        ldap_search_options = ldap_search_options.sizelimit(size_limit);
    }
    let search_stream = ldap
        .with_search_options(ldap_search_options)
        .streaming_search(&opts.basedn, opts.scope, &opts.filter, &opts.attributes)
        .await
        .map_err(|err| LabeledError::new(format!("LDAP error: {}", err)))?;
    Ok(search_stream)
}

#[instrument]
fn shape_entry(entry: &SearchEntry) -> Result<Value, LabeledError> {
    let mut record = Record::new();
    record.insert("dn", Value::string(&entry.dn, Span::unknown()));

    let mut attrs = entry
        .attrs
        .iter()
        .map(|(name, values)| {
            let nu_values = values
                .iter()
                .map(|s| Value::string(s.clone(), Span::unknown()))
                .collect::<Vec<_>>();
            (name.clone(), nu_values)
        })
        .collect::<Vec<_>>();
    attrs.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    let attrs_record = attrs
        .into_iter()
        .map(|(name, values)| (name, Value::list(values, Span::unknown())))
        .collect::<Record>();
    record.insert("attrs", Value::record(attrs_record, Span::unknown()));

    let mut bin_attrs = entry
        .bin_attrs
        .iter()
        .map(|(name, values)| {
            let nu_values = values
                .iter()
                .map(|s| Value::binary(s.clone(), Span::unknown()))
                .collect::<Vec<_>>();
            (name.clone(), nu_values)
        })
        .collect::<Vec<_>>();
    bin_attrs.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    let bin_attrs_record = bin_attrs
        .into_iter()
        .map(|(name, values)| (name, Value::list(values, Span::unknown())))
        .collect::<Record>();
    record.insert(
        "bin_attrs",
        Value::record(bin_attrs_record, Span::unknown()),
    );

    Ok(Value::record(record, Span::unknown()))
}

#[instrument]
pub async fn ldap_client(plugin: &LdapPlugin, opts: &ConnectOpts) -> Result<Ldap, LabeledError> {
    let mut settings = LdapConnSettings::new().set_starttls(opts.starttls);
    if let Some(connect_timeout) = opts.connect_timeout {
        settings = settings.set_conn_timeout(connect_timeout);
    }
    let (conn, mut ldap) = LdapConnAsync::from_url_with_settings(settings, &opts.uri)
        .await
        .map_err(|e| LabeledError::new(format!("LDAP connection error {e}")))?;
    // Duplicate ldap3::drive!() but against LdapPlugin's spawn()
    plugin.spawn(async move {
        if let Err(e) = conn.drive().await {
            warn!(error = ?e);
        }
        Ok(())
    });
    if let Some((binddn, bindpw)) = opts.bind_credentials.as_ref() {
        trace!(event = "simple bind", binddn = binddn);
        ldap.simple_bind(binddn, bindpw)
            .await
            .map_err(|e| LabeledError::new(format!("LDAP bind error {e}")))?;
    }
    Ok(ldap)
}
