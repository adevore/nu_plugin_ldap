# `nu_plugin_ldap`

A [Nushell](https://www.nushell.sh/) plugin for querying LDAP servers. Currently
just search is supported. Uses streams and paged searches to process large result
sets efficiently.

## Usage

```
ldap search [options] <filter> [attributes]...
```

### Options

| Option            | Type   | Description                                       |
| ----------------- | ------ | ------------------------------------------------- |
| -h, --help        |        | Show help                                         |
| `--uri`           | string | LDAP server URI                                   |
| `--binddn`        | string | Bind DN                                           |
| `--basedn`        | string | Base DN for                                       |
| `--scope`         | string | Search scope (base, one, sub)                     |
| `--size-limit`    | number | Page size for paged search                        |
| `--time-limit`    | number | Timeout in seconds                                |
| `--deref-aliases` | string | Dereference aliases (never, find, search, always) |
| `--typesonly`     |        | Return only attribute types                       |

## Output

The output is a list of records, each with this structure:

| Attribute | Type                               | Description                     |
| --------- | ---------------------------------- | ------------------------------- |
| dn        | string                             | Distinguished Name              |
| attrs     | record[list[string]] | List of attribute values        |
| bin_attrs | record[list[bytes]]  | List of binary attribute values |
