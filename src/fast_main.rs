use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Rustoriumを高速モードで起動しています...");
    
    // APIサーバーのみを起動（バックグラウンドで）
    let api_handle = thread::spawn(|| {
        Command::new("cargo")
            .args(&["run", "--bin", "api"])
            .current_dir("api")
            .spawn()
            .expect("APIサーバーの起動に失敗しました");
        
        println!("APIサーバーを起動しました: http://localhost:50128");
    });
    
    // 少し待機
    thread::sleep(Duration::from_secs(1));
    
    println!("Rustoriumの高速起動が完了しました！");
    println!("フロントエンドが必要な場合は、別のターミナルで以下を実行してください:");
    println!("cd frontend && cargo run");
    
    // APIサーバーの終了を待機
    api_handle.join().unwrap();
}