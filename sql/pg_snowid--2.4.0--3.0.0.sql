-- Upgrade script for pg_snowid from version 2.4.0 to 3.0.0
-- Adds SnowID 3.0 non-blocking and batch generation functions.

CREATE FUNCTION "snowid_generate_batch"(
    "table_id" oid,
    "count" INT
) RETURNS bigint[]
STRICT
LANGUAGE c
AS 'MODULE_PATHNAME', 'snowid_generate_batch_wrapper';

CREATE FUNCTION "snowid_generate_batch_int"(
    "table_id" INT,
    "count" INT
) RETURNS bigint[]
STRICT
LANGUAGE c
AS 'MODULE_PATHNAME', 'snowid_generate_batch_int_wrapper';

CREATE FUNCTION "snowid_try_generate"(
    "table_id" oid
) RETURNS bigint
STRICT
LANGUAGE c
AS 'MODULE_PATHNAME', 'snowid_try_generate_wrapper';

CREATE FUNCTION "snowid_try_generate_int"(
    "table_id" INT
) RETURNS bigint
STRICT
LANGUAGE c
AS 'MODULE_PATHNAME', 'snowid_try_generate_int_wrapper';

CREATE FUNCTION "snowid_try_generate_base62"(
    "table_id" oid
) RETURNS TEXT
STRICT
LANGUAGE c
AS 'MODULE_PATHNAME', 'snowid_try_generate_base62_wrapper';

CREATE FUNCTION "snowid_try_generate_base62_int"(
    "table_id" INT
) RETURNS TEXT
STRICT
LANGUAGE c
AS 'MODULE_PATHNAME', 'snowid_try_generate_base62_int_wrapper';

CREATE FUNCTION "snowid_try_generate_batch"(
    "table_id" oid,
    "count" INT
) RETURNS bigint[]
STRICT
LANGUAGE c
AS 'MODULE_PATHNAME', 'snowid_try_generate_batch_wrapper';

CREATE FUNCTION "snowid_try_generate_batch_int"(
    "table_id" INT,
    "count" INT
) RETURNS bigint[]
STRICT
LANGUAGE c
AS 'MODULE_PATHNAME', 'snowid_try_generate_batch_int_wrapper';
