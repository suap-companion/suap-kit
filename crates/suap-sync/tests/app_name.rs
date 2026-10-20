//! A process of its own: with another application name, the names the library derives follow it.

use suap_core::{set_app_name, SyncSettings};
use suap_sync::{
    access_key_env, backend_from, key_env, passphrase_env, secret_key_env, S3Credentials,
};

#[test]
fn environment_variables_and_messages_carry_the_application_name() {
    set_app_name("base-conhecimento").unwrap();
    assert_eq!(key_env(), "BASE_CONHECIMENTO_SYNC_KEY");
    assert_eq!(passphrase_env(), "BASE_CONHECIMENTO_SYNC_PASSPHRASE");
    assert_eq!(access_key_env(), "BASE_CONHECIMENTO_S3_ACCESS_KEY_ID");
    assert_eq!(secret_key_env(), "BASE_CONHECIMENTO_S3_SECRET_ACCESS_KEY");

    let env = |name: &str| (name == "BASE_CONHECIMENTO_S3_ACCESS_KEY_ID").then(|| "id".to_owned());
    assert!(S3Credentials::from_env(&env).is_none());

    let settings = SyncSettings {
        backend: Some("s3".to_owned()),
        endpoint: Some("https://x.example".to_owned()),
        bucket: Some("b".to_owned()),
        ..SyncSettings::default()
    };
    let message = backend_from(&settings, None).err().unwrap().to_string();
    assert!(
        message.contains("BASE_CONHECIMENTO_S3_ACCESS_KEY_ID"),
        "{message}"
    );
    assert!(
        message.contains("`base-conhecimento sync credentials set`"),
        "{message}"
    );
}
