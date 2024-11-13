# sqlite-extras-rs [![Latest Version](https://img.shields.io/crates/v/sqlite-extras.svg)](https://crates.io/crates/sqlite-extras) [![GH Actions](https://github.com/pawurb/sqlite-extras-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/pawurb/sqlite-extras-rs/actions)

Copy/paste of [ecto_sqlite3_extras](https://github.com/orsinium-labs/ecto_sqlite3_extras). Helper queries providing insights into the Sqlite database.

## Installation

```bash
cargo install sqlite-extras
```

## Usage

Package expects the `$SQLITE_EXTRAS_DATABASE_URL` or `$DATABASE_URL` env in the following format:

```rust
export DATABASE_URL="sqlite://your_database.db"
```

You can use `sqlextras` shell command:

```
Usage: sqlextras <COMMAND>

Commands:
  table-size       Metadata of the tables (excluding indexes), descending by size
  index-size       Metadata of the indexes, descending by size
  integrity-check  Run integrity checks on the database
  pragma           List values of PRAGMAs (settings)
  total-size       The total size of all tables and indexes
  compile-options  List the compile-time options used when building SQLite
  sequence-number  Sequence numbers of autoincrement columns
  help             Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version
```

## Available queries

### `total_size`

The total size of all tables and indices. It's a summary table, it has only 2 columns: `name` and `value`. Rows:

- `cells`: The number of cells in the DB. Each value stored in the DB is represented as at least one cell. So, the number of cells correlates with the number of records in the DB.
- `payload_size`: How much space the actual useful payload takes in the DB.
- `unused_size`: How much space in the DB is reserved, not used yet, and can be used later to store more data. This is a surplus that occurs because SQLite allocates space for data in chunks ("pages").
- `vacuum_size`: How much space is unused and cannot be used for future data. You can run [VACUUM](https://www.sqlite.org/lang_vacuum.html) command to reduce it.
- `page_size`: The total space occupied by all pages. Each page is a single chunk of space allocated by SQLite. This number is the sum of `payload_size`, `unused_size`, and `vacuum_size`.
- `pages`: The total number of pages.
- `pages: leaf`: The pages that store the actual data. Read [SQLite Internals: Pages & B-trees](https://fly.io/blog/sqlite-internals-btree/) to learn more.
- `pages: internal`: The pages that store ranges for leaf pages for a faster lookup. Sometimes also called "interior pages".
- `pages: overflow`: The pages that store chunks of big data that don't fit in a single leaf page.
- `pages: table`: The pages used for storing data for tables.
- `pages: index`: The pages used for storing indices.

### `table_size`

Information about the space used (and unused) by all tables. Based on the [dbstat](https://www.sqlite.org/dbstat.html) virtual table.

- `name`: The table name.
- `payload_size`.
- `unused_size`.
- `vacuum_size`.
- `page_size`.
- `cells`.
- `pages`.
- `max_payload_size`: The size of the biggest payload in the table.

### `index_size`

Size of all indices.

- `name`: The index name.
- `table_name`: The table where the index is defined.
- `column_name`: The name of the column being indexed. This column is NULL if the column is the rowid or an expression.
- `payload_size`.
- `unused_size`.
- `page_size`.
- `cells`.
- `pages`.
- `max_payload_size`.

### `sequence_number`

Sequence numbers of autoincrement columns. Generated based on the [sqlite_sequence](https://renenyffenegger.ch/notes/development/databases/SQLite/internals/schema-objects/sqlite_sequence) table. The query will fail if there are no autoincrement columns in the DB yet.

- `table_name`.
- `sequence_number`.

### `pragma`

List values of PRAGMAs (settings). Only includes the ones that have an integer or a boolean value. For brevity, the ones with the `0` (`false`) value are excluded from the output (based on the observation that this is the default value for most of the PRAGMAs). Check out the SQLite documentation to learn more about what each PRAGMA means: [PRAGMA Statements](https://www.sqlite.org/pragma.html).

- `name`: the name of the PRAGMA as listed in the SQLite documentation.
- `value`: the value of the PRAGMA. The `true` value is converted into `1` (and `false` is simply excluded).

### `compile_options` 

List the [compile-time options](https://www.sqlite.org/compile.html) used when building SQLite, one option per row. The "SQLITE_" prefix is omitted from the returned option names. See [exqlite docs](https://github.com/elixir-sqlite/exqlite#defining-extra-compile-flags) on how to change these options.

### `integrity_check`

Run integrity checks on the database. Executes [PRAGMA integrity_check](https://www.sqlite.org/pragma.html#pragma_integrity_check) and returns the resulting messages.