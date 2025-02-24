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

## 🎯 Installation

### Build from source

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Install PostgreSQL development packages
3. Install [pgrx](https://github.com/pgcentralfoundation/pgrx):
```bash
cargo install --locked cargo-pgrx
cargo pgrx init --pg17 download
```

4. Clone and build the extension:
```bash
git clone https://github.com/qeeqez/snowid-postgres.git
cd snowid-postgres
cargo pgrx install
```

## 📊 Usage

After installation, enable the extension in your database:

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
-- Create a table with SnowID
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
```

### Extract ID Components

```sql
-- Extract timestamp from ID
SELECT snowid_get_timestamp(151819733950271234, 1);  -- Pass table_id

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
