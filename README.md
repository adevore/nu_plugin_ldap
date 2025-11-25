# `nu_plugin_ldap`

A [Nushell](https://www.nushell.sh/) plugin for querying LDAP servers.
Uses streams and paged searches to process large result sets efficiently.

## Output

The output is a list of records, each with this structure:

| Attribute | Type | Description |
|-----------|------|-------------|
| dn        | string | Distinguished Name |
| attrs     | list[record[string, list[string]]] | List of attribute values |
| bin_attrs | list[record[string, list[bytes]]] | List of binary attribute values |
