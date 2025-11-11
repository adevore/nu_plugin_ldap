use ldap3::LdapConnAsync;

use super::opts::Opts;

pub(crate) async fn search(opts: Opts) {
    /*
    let uri = opts
        .uri
        .take()
        .unwrap_or_else(|| "ldap://localhost".to_owned());
    let (conn, mut ldap) = LdapConnAsync::new(&uri).await?;
    let conn = client.connect().await?;
    let result = conn
        .search(&opts.basedn, opts.scope, &opts.filter, &opts.attributes)
        .await?;
    println!("{:?}", result);
    */
}
