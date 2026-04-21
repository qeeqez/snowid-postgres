use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const EXTENSION_NAME: &str = "pg_snowid";

#[test]
#[ignore = "requires cargo pgrx package to run first"]
fn packaged_build_contains_versioned_sql_files() {
    let package_dir = env::var_os("PG_SNOWID_PACKAGE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/release/pg_snowid-pg18"));

    assert!(
        package_dir.is_dir(),
        "package directory does not exist: {}",
        package_dir.display()
    );

    let extension_dir = find_extension_dir(&package_dir).unwrap_or_else(|| {
        panic!(
            "could not find {EXTENSION_NAME}.control under {}",
            package_dir.display()
        )
    });

    let source_sql_files = sql_file_names(Path::new("sql"));
    assert!(
        !source_sql_files.is_empty(),
        "no source SQL files found under sql/"
    );

    for file in &source_sql_files {
        assert!(
            is_valid_pg_snowid_sql_name(file),
            "source SQL file has invalid name: sql/{file}"
        );
    }

    let packaged_sql_files = sql_file_names(&extension_dir);
    assert!(
        !packaged_sql_files.is_empty(),
        "no packaged {EXTENSION_NAME} SQL files found in {}",
        extension_dir.display()
    );

    for file in &packaged_sql_files {
        assert!(
            is_valid_pg_snowid_sql_name(file),
            "packaged SQL file has invalid name: {}/{file}",
            extension_dir.display()
        );
    }

    for file in &source_sql_files {
        assert!(
            extension_dir.join(file).is_file(),
            "package is missing SQL file from sql/: {file}"
        );
    }

    let default_version = default_version_from_control(Path::new("pg_snowid.control"));
    let install_sql = format!("{EXTENSION_NAME}--{default_version}.sql");
    assert!(
        extension_dir.join(&install_sql).is_file(),
        "package is missing generated install SQL file: {install_sql}"
    );
}

fn find_extension_dir(package_dir: &Path) -> Option<PathBuf> {
    let control_file = format!("{EXTENSION_NAME}.control");
    find_file(package_dir, &control_file).map(|path| {
        path.parent()
            .expect("control file path should have parent directory")
            .to_path_buf()
    })
}

fn find_file(dir: &Path, file_name: &str) -> Option<PathBuf> {
    for entry in fs::read_dir(dir).unwrap_or_else(|err| {
        panic!("failed to read directory {}: {err}", dir.display());
    }) {
        let entry = entry.unwrap_or_else(|err| {
            panic!("failed to read entry under {}: {err}", dir.display());
        });
        let path = entry.path();

        if path.is_dir() {
            if let Some(found) = find_file(&path, file_name) {
                return Some(found);
            }
        } else if path.file_name().is_some_and(|name| name == file_name) {
            return Some(path);
        }
    }

    None
}

fn sql_file_names(dir: &Path) -> Vec<String> {
    let mut files = fs::read_dir(dir)
        .unwrap_or_else(|err| panic!("failed to read directory {}: {err}", dir.display()))
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            let file_name = path.file_name()?.to_str()?;

            if path.is_file()
                && file_name.starts_with(EXTENSION_NAME)
                && file_name.ends_with(".sql")
            {
                Some(file_name.to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    files.sort();
    files
}

fn is_valid_pg_snowid_sql_name(file_name: &str) -> bool {
    let Some(version_part) = file_name
        .strip_prefix(&format!("{EXTENSION_NAME}--"))
        .and_then(|name| name.strip_suffix(".sql"))
    else {
        return false;
    };

    let versions = version_part.split("--").collect::<Vec<_>>();
    matches!(versions.len(), 1 | 2) && versions.into_iter().all(is_semver_triplet)
}

fn is_semver_triplet(version: &str) -> bool {
    let parts = version.split('.').collect::<Vec<_>>();
    parts.len() == 3
        && parts
            .iter()
            .all(|part| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit()))
}

fn default_version_from_control(path: &Path) -> String {
    let control = fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));

    for line in control.lines() {
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };

        if key.trim() != "default_version" {
            continue;
        }

        let value = value.trim();
        let version = if let Some(value) = value.strip_prefix('\'') {
            value.split('\'').next().unwrap_or_default()
        } else if let Some(value) = value.strip_prefix('"') {
            value.split('"').next().unwrap_or_default()
        } else {
            value.split_whitespace().next().unwrap_or_default()
        };
        assert!(
            is_semver_triplet(version),
            "{} does not contain a valid x.x.x default_version",
            path.display()
        );
        return version.to_owned();
    }

    panic!("{} does not contain default_version", path.display());
}
