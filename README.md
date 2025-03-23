# ❄️ SnowID PostgreSQL Extension

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> A PostgreSQL extension for generating Snowflake-like IDs using [snowid](https://crates.io/crates/snowid) Rust library.

**Generate 64-bit unique identifiers in PostgreSQL that are:**
- ⚡️ Fast (~244ns per ID)
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

Use our pre-built PostgreSQL 17 image with SnowID extension:

```bash
docker pull qeeqez/snowid:v0.1.0-pg17
docker run -e POSTGRES_PASSWORD=postgres -p 5432:5432 qeeqez/snowid:v0.1.0-pg17
```

The image comes with:
- PostgreSQL 17
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
