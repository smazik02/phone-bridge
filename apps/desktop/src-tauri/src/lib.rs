use std::fs;
use keyring::Entry;
use rcgen::{generate_simple_self_signed, CertifiedKey};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_or_create_identity() -> (String, String) {
    let service = "com.smazik.phonebridge";
    let user = "identity_private_key";
    let entry = Entry::new(service, user).expect("Failed to create keyring entry");

    if let Ok(stored_key) = entry.get_password() {
        let cert_pem = read_certificate_pem();
        return (cert_pem, stored_key);
    }

    let subject_alt_names = vec!["localhost".to_string(), "desktop-bridge.local".to_string()];
    let CertifiedKey { cert, signing_key } =
        generate_simple_self_signed(subject_alt_names).expect("Failed to generate key");

    let cert_pem = cert.pem();
    let key_pem = signing_key.serialize_pem();

    entry.set_password(&key_pem).expect("Failed to store key in Keychain/Vault");

    fs::write("cert.pem", &cert_pem).expect("Failed to save certificate file");

    (cert_pem, key_pem)
}

fn read_certificate_pem() -> String {
    fs::read_to_string("cert.pem").expect("Failed to load certificate PEM")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
