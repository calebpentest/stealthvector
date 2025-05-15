const XOR_KEY: u8 = 0xAA;

fn xor_encrypt(data: &str) -> Vec<u8> {
    data.bytes().map(|b| b ^ XOR_KEY).collect()
}

fn main() {
    let bot_token = "7602516714:AAFFlw7S-y_lJJCV2zdGCPSvsMKY0wBAzPU";
    let chat_id = "7217605816";

    let encrypted_token = xor_encrypt(bot_token);
    let encrypted_chat_id = xor_encrypt(chat_id);

    println!("Encrypted bot token bytes: {:?}", encrypted_token);
    println!("Encrypted chat ID bytes: {:?}", encrypted_chat_id);
}