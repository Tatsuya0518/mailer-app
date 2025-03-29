// Prevents additional console window on Windows in release, DO NOT REMOVE!!
/*#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    mailer_app_lib::run()
}

 */


 // main.rs の上部に追加
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

//use tauri::Manager; // 必要に応じて追加

// --- メール送信関連の use 文を追加 ---
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use dotenv::dotenv;
use std::env;
// --- ここまで追加 ---

// フロントエンドから受け取るデータの型定義
#[derive(serde::Deserialize)]
struct EmailPayload {
    to: String,
    subject: String,
    body: String,
}

// Tauriコマンドとしてメール送信関数を定義
#[tauri::command]
async fn send_email(payload: EmailPayload) -> Result<(), String> {
    // .env ファイルを読み込む (main関数で一度だけ呼ぶのが一般的だが、コマンド内でも可)
    // dotenv().ok(); // main関数で呼ぶ場合は不要

    // 環境変数から設定を読み込む
    let smtp_username = env::var("SMTP_USERNAME").map_err(|e| format!("Missing SMTP_USERNAME: {}", e))?;
    let smtp_password = env::var("SMTP_PASSWORD").map_err(|e| format!("Missing SMTP_PASSWORD: {}", e))?;
    let smtp_host = env::var("SMTP_HOST").map_err(|e| format!("Missing SMTP_HOST: {}", e))?;
    let from_email = env::var("FROM_EMAIL").map_err(|e| format!("Missing FROM_EMAIL: {}", e))?;

    // メールメッセージを作成
    let email = Message::builder()
        .from(from_email.parse().map_err(|e| format!("Invalid FROM_EMAIL: {}", e))?)
        .to(payload.to.parse().map_err(|e| format!("Invalid 'to' address: {}", e))?)
        .subject(payload.subject)
        // HTMLメールを送りたい場合は .header(ContentType::TEXT_HTML) と .body(html_content) を使う
        .body(payload.body)
        .map_err(|e| format!("Failed to build email: {}", e))?;

    // SMTP認証情報
    let creds = Credentials::new(smtp_username.to_owned(), smtp_password.to_owned());

    // SMTPトランスポーターを作成 (TLS接続)
    // lettre v0.11以降、トランスポーターの作成は同期的
    let mailer = SmtpTransport::relay(&smtp_host)
        .map_err(|e| format!("Failed to create relay: {}", e))?
        .credentials(creds)
        .build();

    // メールを送信 (lettre v0.11の Transport::send は同期的)
    // Tauriのコマンドハンドラは非同期(`async fn`)なので、ブロッキングする可能性のある
    // 同期処理は tokio::spawn_blocking でラップするのがより安全ですが、
    // 短時間で終わる処理なら直接呼び出しても問題ない場合が多いです。
    // 長時間かかる場合は spawn_blocking を検討してください。

    // spawn_blocking を使う例（コマンドハンドラ内）
    // tokio::task::spawn_blocking(move || {
    //     mailer.send(&email)
    // }).await.map_err(|e| format!("Task join error: {}", e))? // Outer Result for join error
    // .map_err(|e| format!("Failed to send email: {}", e)) // Inner Result from send()

    match mailer.send(&email) {
        Ok(_) => {
            println!("Email sent successfully!");
            Ok(())
        }
        Err(e) => {
            eprintln!("Could not send email: {:?}", e);
            Err(format!("Failed to send email: {}", e))
        }
    }
}


fn main() {
    // .env ファイルをアプリケーション起動時に読み込む
    dotenv().ok();

    tauri::Builder::default()
        // 作成したコマンドをハンドラに登録
        .invoke_handler(tauri::generate_handler![send_email])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}