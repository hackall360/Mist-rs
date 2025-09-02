use mist_core::model::architecture::Architecture;
use mist_core::model::chunk::Chunk;
use mist_core::model::chunklist::Chunklist;
use mist_core::model::firmware::Firmware;
use mist_core::model::hardware::Hardware;
use mist_core::model::installer::Installer;
use mist_core::model::mist_error::MistError;
use mist_core::model::mist_task::MistTask;
use mist_core::model::mist_task_section::MistTaskSection;
use mist_core::model::mist_task_state::MistTaskState;
use mist_core::model::mist_task_type::MistTaskType;
use mist_core::model::package::Package;
use mist_core::model::progress_alert_type::ProgressAlertType;
use mist_core::model::refresh_state::RefreshState;
use mist_core::model::settings_installer_cache_alert_type::SettingsInstallerCacheAlertType;
use serde::de::DeserializeOwned;
use serde::Serialize;
use uuid::Uuid;

fn roundtrip_json<T>(value: &T)
where
    T: Serialize + DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let json = serde_json::to_string(value).expect("serialize to json");
    let de: T = serde_json::from_str(&json).expect("deserialize from json");
    assert_eq!(&de, value);
}

fn roundtrip_yaml<T>(value: &T)
where
    T: Serialize + DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let yaml = serde_yaml::to_string(value).expect("serialize to yaml");
    let de: T = serde_yaml::from_str(&yaml).expect("deserialize from yaml");
    assert_eq!(&de, value);
}

fn roundtrip_plist<T>(value: &T)
where
    T: Serialize + DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let mut bytes = Vec::new();
    plist::to_writer_xml(&mut bytes, value).expect("serialize to plist");
    let de: T = plist::from_bytes(&bytes).expect("deserialize from plist");
    assert_eq!(&de, value);
}

#[test]
fn model_roundtrip() {
    let chunk = Chunk {
        size: 10,
        hash: vec![0xaa, 0xbb],
    };
    roundtrip_json(&chunk);
    roundtrip_yaml(&chunk);
    roundtrip_plist(&chunk);

    let chunklist = Chunklist {
        magic_header: 0x1,
        header_size: 0x24,
        file_version: 1,
        chunk_method: 1,
        signature_method: 2,
        padding: 0,
        total_chunks: 1,
        chunks_offset: 0x24,
        signature_offset: 0x48,
        chunks: vec![chunk.clone()],
        signature: vec![0xcc],
    };
    roundtrip_json(&chunklist);
    roundtrip_yaml(&chunklist);
    roundtrip_plist(&chunklist);

    let firmware = Firmware {
        version: "13.0".into(),
        build: "22A380".into(),
        shasum: "abcd".into(),
        size: 1234,
        url: "https://example.com/firmware".into(),
        date: "2022-10-24".into(),
        signed: true,
        compatible: true,
    };
    roundtrip_json(&firmware);
    roundtrip_yaml(&firmware);
    roundtrip_plist(&firmware);

    let hardware = Hardware {
        architecture: Some(Architecture::AppleSilicon),
        board_id: Some("board".into()),
        device_id: Some("device".into()),
        model_identifier: Some("model".into()),
    };
    roundtrip_json(&hardware);
    roundtrip_yaml(&hardware);
    roundtrip_plist(&hardware);

    let package = Package {
        url: "https://example.com/pkg".into(),
        size: 100,
        integrity_data_url: None,
        integrity_data_size: None,
    };
    let installer = Installer {
        id: "123".into(),
        version: "1.0".into(),
        build: "BUILD".into(),
        date: "2022-01-01".into(),
        distribution_url: "https://example.com/dist".into(),
        distribution_size: 42,
        packages: vec![package],
        board_ids: vec!["BOARD".into()],
        device_ids: vec!["DEV".into()],
        unsupported_model_identifiers: vec![],
    };
    roundtrip_json(&installer);
    roundtrip_yaml(&installer);
    roundtrip_plist(&installer);

    let err = MistError::InvalidFileSize {
        invalid: 1,
        valid: 2,
    };
    roundtrip_json(&err);
    roundtrip_yaml(&err);
    roundtrip_plist(&err);

    let task = MistTask {
        id: Uuid::new_v4(),
        r#type: MistTaskType::Download,
        state: MistTaskState::Pending,
        description: "Test".into(),
        download_size: Some(50),
    };
    roundtrip_json(&task);
    roundtrip_yaml(&task);
    roundtrip_plist(&task);

    let section = MistTaskSection::Download;
    roundtrip_json(&section);
    roundtrip_yaml(&section);
    roundtrip_plist(&section);

    let state = MistTaskState::Complete;
    roundtrip_json(&state);
    roundtrip_yaml(&state);
    roundtrip_plist(&state);

    let typ = MistTaskType::Verify;
    roundtrip_json(&typ);
    roundtrip_yaml(&typ);
    roundtrip_plist(&typ);

    let alert = ProgressAlertType::Cancel;
    roundtrip_json(&alert);
    roundtrip_yaml(&alert);
    roundtrip_plist(&alert);

    let refresh = RefreshState::Warning;
    roundtrip_json(&refresh);
    roundtrip_yaml(&refresh);
    roundtrip_plist(&refresh);

    let cache_alert = SettingsInstallerCacheAlertType::Confirmation;
    roundtrip_json(&cache_alert);
    roundtrip_yaml(&cache_alert);
    roundtrip_plist(&cache_alert);
}
