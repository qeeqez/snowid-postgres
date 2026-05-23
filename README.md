# ❄️ SnowID PostgreSQL Extension

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> A PostgreSQL extension for generating Snowflake-like IDs using [snowid](https://crates.io/crates/snowid) Rust library.

**Generate 64-bit unique identifiers in PostgreSQL that are:**
- ⚡️ Fast with SnowID 3.0 logical timestamp generation
- 📈 Time-sorted
- 🔄 Monotonic
- 🔒 Thread-safe
- 🌐 Distributed-ready

## 🧮 ID Structure

**Example ID**: 151819733950271234

**Default configuration:**
```text
|------------------------------------------|------------|------------|
|           TIMESTAMP (42 bits)            | NODE (10)  |  SEQ (12)  |
|------------------------------------------|------------|------------|
```
- Timestamp: 42 bits = 139 years from 2024-01-01 (1704067200000)
- Node ID: 10 bits = 1,024 nodes (valid range: 6-16 bits)
- Sequence: 12 bits = 4,096 IDs/ms/node

**Base62 Representation:**
```text
Example: "2qPfVQh7Jw9"
```
- Uses characters 0-9, a-z, A-Z for more compact, URL-friendly IDs
- Maximum length: 11 characters for any 64-bit integer
- Preserves time-sorting property of numeric IDs

## 🎯 Installation

<details>
<summary><b>Docker Image</b></summary>

Use our pre-built PostgreSQL 18 image with SnowID extension:

```bash
docker pull rixl/snowid-pg:18
docker run -e POSTGRES_PASSWORD=postgres -p 5432:5432 rixl/snowid-pg:18
```

The image comes with:
- PostgreSQL 18
- SnowID extension installed
- `shared_preload_libraries` configured
</details>

<details>
<summary><b>Manual Installation</b></summary>

1. Build and install the extension:
```bash
cargo pgrx install --release
```

2. Add the extension to `postgresql.conf`:
```ini
# Required: Add pg_snowid to shared_preload_libraries
shared_preload_libraries = 'pg_snowid'
```

3. Restart PostgreSQL server to load the library
</details>

## 📊 Usage

First, create the extension in your database:
```sql
CREATE EXTENSION pg_snowid;
```

### Set Node ID (Optional)

```sql
-- Set node ID (0-1023, default is 1)
SELECT snowid_set_node(5);

-- Get current node ID
SELECT snowid_get_node();
```

If you do not call `snowid_set_node`, the extension uses the fixed default node ID `1`. This value is not auto-generated from the host or PostgreSQL instance. In multi-node deployments, set a unique node ID on every instance before generating IDs; otherwise different instances using the same node ID can produce overlapping ID ranges.

### Create Table with SnowID

```sql
-- Create a table with SnowID (numeric format)
CREATE TABLE users (
    id bigint PRIMARY KEY DEFAULT snowid_generate(1),  -- Use unique table_id (1)
    name text,
    created_at timestamptz DEFAULT current_timestamp
);

-- Create another table with SnowID
CREATE TABLE posts (
    id bigint PRIMARY KEY DEFAULT snowid_generate(2),  -- Use different table_id (2)
    title text,
    content text,
    created_at timestamptz DEFAULT current_timestamp
);

-- Create a table with base62-encoded SnowID
CREATE TABLE products (
    id VARCHAR(11) PRIMARY KEY DEFAULT snowid_generate_base62(3),  -- Use unique table_id (3)
    name text,
    price numeric,
    created_at timestamptz DEFAULT current_timestamp
);
```

> **Note**: Each table requires a unique positive integer ID (1-1024). The extension currently supports up to 1024 tables. If you need support for more tables, please [create an issue](https://github.com/qeeqez/snowid-postgres/issues) and we'll add this functionality.

### Extract ID Components

```sql
-- Extract timestamp from numeric ID
SELECT snowid_get_timestamp(151819733950271234);

-- Extract timestamp from base62 ID
SELECT snowid_get_timestamp_base62('2qPfVQh7Jw9');

-- View SnowID statistics
SELECT snowid_stats();
```

### PostgreSQL Functions

| Function | Returns | Description |
| --- | --- | --- |
| `snowid_set_node(node smallint)` | `void` | Sets the node ID for this PostgreSQL instance before generators are created. |
| `snowid_get_node()` | `smallint` | Returns the current node ID. |
| `snowid_generate(table_id oid)` | `bigint` | Generates one logical timestamp SnowID. |
| `snowid_generate_base62(table_id oid)` | `text` | Generates one logical timestamp SnowID encoded as Base62. |
| `snowid_try_generate(table_id oid)` | `bigint` or `NULL` | Generates one ID only when available without logical timestamp advancement. |
| `snowid_try_generate_base62(table_id oid)` | `text` or `NULL` | Generates one Base62 ID only when available without logical timestamp advancement. |
| `snowid_generate_batch(table_id oid, count int)` | `bigint[]` | Generates `count` IDs with logical batch reservation. |
| `snowid_try_generate_batch(table_id oid, count int)` | `bigint[]` | Returns only IDs immediately available without logical timestamp advancement. |
| `snowid_get_timestamp(id bigint)` | `bigint` | Extracts the timestamp component from a numeric SnowID. |
| `snowid_get_timestamp_base62(encoded_id text)` | `bigint` | Extracts the timestamp component from a Base62 SnowID. |
| `snowid_stats()` | `text` | Returns generator and node statistics. |

### High-Load Generation Behavior

`pg_snowid` uses SnowID 3.0's default logical timestamp generation through `snowid_generate(...)` and `snowid_generate_base62(...)`.

When one generator exhausts the current millisecond's sequence range, it no longer waits for the next wall-clock millisecond. Instead, it advances the timestamp component logically and returns an ID immediately. IDs remain unique, time-sorted, and monotonic for that generator, while the embedded timestamp can run ahead of wall-clock time during sustained overload.

This removes the previous hot-path wait under bursty write load and can significantly improve generation throughput. Keep table IDs distributed across tables or shards when possible, because each table ID maps to its own shared generator.

If you need wall-clock-only generation, use the `try_*` functions. They return `NULL` instead of advancing logical time when the current millisecond is exhausted or when the generator is already ahead of wall-clock time:

```sql
SELECT snowid_try_generate(1);
SELECT snowid_try_generate_base62(1);
```

For bulk inserts or reservation-heavy paths, use batch generation. `snowid_generate_batch` always returns the requested number of IDs and may advance logical time. `snowid_try_generate_batch` returns only the IDs that can be reserved immediately without logical timestamp advancement.

```sql
SELECT unnest(snowid_generate_batch(1, 1000));
SELECT unnest(snowid_try_generate_batch(1, 1000));
```

## 🔧 Development

```bash
# Run tests
cargo pgrx test

# Package the extension
cargo pgrx package

# Install the extension
cargo pgrx install
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [pgrx](https://github.com/pgcentralfoundation/pgrx)
- Uses [snowid](https://crates.io/crates/snowid) Rust library
