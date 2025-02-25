# CloudNative PG Custom Image with pg_snowid Extension

This is a custom PostgreSQL image built for CloudNative PG that includes the pg_snowid extension. The image is based on the official CloudNative PG image `ghcr.io/cloudnative-pg/postgresql:17.4-bookworm` and includes pre-built pg_snowid extension.

## Image Details

- Base Image: `ghcr.io/cloudnative-pg/postgresql:17.4-bookworm`
- PostgreSQL Version: 17.4
- Included Extension: pg_snowid

## Usage

To use this image with CloudNative PG, you need to ensure that the `pg_snowid` extension is properly loaded using shared preload libraries. There are two ways to configure this:

### 1. Manual Configuration (postgresql.conf)

If you're providing your own `postgresql.conf`, make sure to include the following line:

```conf
shared_preload_libraries = 'pg_snowid'
```

### 2. Cluster Configuration (Recommended)

When using CloudNative PG operator, you can configure shared preload libraries in your cluster specification. Add the following to your cluster configuration:

```yaml
apiVersion: postgresql.cnpg.io/v1
kind: Cluster
metadata:
  name: your-cluster-name
spec:
  postgresql:
    shared_preload_libraries:
      - pg_snowid
```

The operator will automatically merge this with other required libraries that it manages.

## Additional Information
- You can't install extension with `CREATE EXTENSION pg_snowid;` by default
- You could temporary **enableSuperuserAccess**:
 ```yaml
  cluster:
    enableSuperuserAccess: true
  ```
- New database could be inited with **postInitApplicationSQL**:
  ```yaml
  cluster:
    initdb:
      postInitApplicationSQL:
        - CREATE EXTENSION pg_snowid;
  ```
- For more details about shared preload libraries configuration in CloudNative PG, refer to the [official documentation](https://cloudnative-pg.io/documentation/1.17/postgresql_conf/#shared-preload-libraries)

### Terraform Example

Here's a complete example of how to configure the extension in Terraform, including both the automatic extension creation and shared library loading:

```hcl
cluster {
  enableSuperuserAccess = true # Enable this or initdb
  initdb = {
    postInitApplicationSQL = ["CREATE EXTENSION pg_snowid;"]
  }
  postgresql = {
    parameters = {
      shared_preload_libraries = ["pg_snowid"]
    }
  }
}
```

## Building the Image

The image is built using a multi-stage Dockerfile that:
1. Builds the pg_snowid extension from source using Rust and pgrx
2. Copies the built extension files into the final CloudNative PG image

## Notes

- Always ensure that your PostgreSQL configuration properly loads the pg_snowid extension
- The extension must be explicitly created in each database where you want to use it
- Monitor the PostgreSQL logs during startup to verify that the extension is loaded correctly
