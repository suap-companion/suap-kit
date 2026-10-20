//! A process of its own: the application name is set once and for good.

use suap_core::{app_env_prefix, app_name, set_app_name, AppPaths, SuapError, DEFAULT_APP_NAME};

#[test]
fn the_name_is_validated_set_once_and_drives_the_data_directory() {
    assert_eq!(app_name(), DEFAULT_APP_NAME);
    for invalid in [
        "",
        "Base",
        "1base",
        "base conhecimento",
        "base_conhecimento",
    ] {
        let error = set_app_name(invalid).unwrap_err();
        assert!(matches!(error, SuapError::InvalidAppName(_)), "{invalid}");
    }
    assert_eq!(app_name(), DEFAULT_APP_NAME);

    set_app_name("base-conhecimento").unwrap();
    set_app_name("base-conhecimento").unwrap();
    assert_eq!(app_name(), "base-conhecimento");
    assert_eq!(app_env_prefix(), "BASE_CONHECIMENTO");
    let other = set_app_name("chamados").unwrap_err().to_string();
    assert!(other.contains("already named base-conhecimento"), "{other}");

    let paths = AppPaths::discover().unwrap();
    assert!(paths
        .data_dir()
        .to_string_lossy()
        .contains("base-conhecimento"));
}
