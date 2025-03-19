        .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏");

    // サービスの起動
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(spinner_style.clone());
    spinner.set_message("Starting services...".to_string());
    spinner.enable_steady_tick(Duration::from_millis(100));

    // APIサーバーの起動
    let api_args = "cargo run".split_whitespace().collect::<Vec<&str>>();
    let _api_process = Command::new(api_args[0])
        .current_dir("api")
        .args(&api_args[1..])
        .args(["--bin", "api"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    // フロントエンドサーバーの起動
    let frontend_args = "cargo run".split_whitespace().collect::<Vec<&str>>();
    let _frontend_process = Command::new(frontend_args[0])
        .current_dir("frontend")
        .args(&frontend_args[1..])
        .args(["--bin", "frontend"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    // サービスの起動を待機
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    spinner.finish_with_message("✨ All services started successfully!");
    println!();

    // インタラクティブコンソールの起動
    AppOptions::interactive_console(&app_state).await?;

    // 終了処理
    println!("\n{}", style("Shutting down services...").yellow());
    let _ = Command::new("pkill")
        .args(["-f", "target/debug/api"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    let _ = Command::new("pkill")
        .args(["-f", "target/debug/frontend"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    println!("{}", style("✨ Thank you for using Rustorium!").green().bold());
    spinner.set_style(spinner_style.clone());
    spinner.set_message("Cleaning up existing processes...".to_string());
    spinner.enable_steady_tick(Duration::from_millis(100));
    
    // APIプロセスのクリーンアップ
    if !options.frontend_only {
        let _ = Command::new("pkill")
            .args(["-f", "target/debug/api"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
            
        let _ = Command::new("pkill")
            .args(["-f", "target/release/api"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
    
    // フロントエンドプロセスのクリーンアップ
    if !options.api_only {
        let _ = Command::new("pkill")
            .args(["-f", "target/debug/frontend"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
            
        let _ = Command::new("pkill")
            .args(["-f", "target/release/frontend"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
    
    // 自分自身のプロセスIDを取得して除外する
    let current_pid = std::process::id();
    info!("Current process ID: {}", current_pid);
    
    // 自分自身以外のrustoriumプロセスを終了
    let _ = Command::new("sh")
        .arg("-c")
        .arg(format!("ps -ef | grep target/debug/rustorium | grep -v {} | grep -v grep | awk '{{print $2}}' | xargs -r kill", current_pid))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
        
    let _ = Command::new("sh")
        .arg("-c")
        .arg(format!("ps -ef | grep target/release/rustorium | grep -v {} | grep -v grep | awk '{{print $2}}' | xargs -r kill", current_pid))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    spinner.finish_with_message("✨ Cleanup completed");
    println!();

    // 起動モードに応じたコマンドを構築
    let api_port = options.api_port.unwrap_or(53036);
    let frontend_port = options.frontend_port.unwrap_or(55938);
    
    let cargo_command = match options.mode {
        ExecutionMode::Production => "cargo run --release",
        ExecutionMode::Development => "cargo run",
        ExecutionMode::Test => "cargo run --profile test",
    };
    
    // APIサーバーを起動
    if matches!(options.services, ServiceMode::All | ServiceMode::ApiOnly) {
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(spinner_style.clone());
        spinner.set_message(format!("Starting API server on port {}...", api_port));
        spinner.enable_steady_tick(Duration::from_millis(100));

        let api_args = cargo_command.split_whitespace().collect::<Vec<&str>>();
        let _api_process = Command::new(api_args[0])
            .current_dir("api")
            .args(&api_args[1..])
            .args(["--bin", "api"])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;

        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        spinner.finish_with_message(format!("🚀 API server started on port {}", api_port));
        println!();
    }
    
    // フロントエンドサーバーを起動
    if matches!(options.services, ServiceMode::All | ServiceMode::FrontendOnly) {
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(spinner_style.clone());
        spinner.set_message(format!("Starting Frontend server on port {}...", frontend_port));
        spinner.enable_steady_tick(Duration::from_millis(100));

        let frontend_args = cargo_command.split_whitespace().collect::<Vec<&str>>();
        let _frontend_process = Command::new(frontend_args[0])
            .current_dir("frontend")
            .args(&frontend_args[1..])
            .args(["--bin", "frontend"])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;

        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        spinner.finish_with_message(format!("🌐 Frontend server started on port {}", frontend_port));
        println!();
    }
    
    println!("\n{}", style("🎉 All services started successfully!").green().bold());
    println!();

    // サーバーの起動を待機（最大10秒）
    for _ in 0..10 {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        
        // 両方のサーバーが起動したかチェック
        let api_ready = options.frontend_only || std::fs::read_to_string("/tmp/api_port").is_ok();
        let frontend_ready = options.api_only || std::fs::read_to_string("/tmp/frontend_port").is_ok();
        
        if api_ready && frontend_ready {
            break;
        }
    }
    
    // サービスURLの表示
    println!("{}", style("Available Services:").cyan().bold());
    if !options.frontend_only {
        if let Ok(port) = std::fs::read_to_string("/tmp/api_port") {
            println!("  {} {}", style("API:").yellow(), style(format!("http://localhost:{}", port)).underlined());
        }
    }
    
    if !options.api_only {
        if let Ok(port) = std::fs::read_to_string("/tmp/frontend_port") {
            println!("  {} {}", style("Frontend:").yellow(), style(format!("http://localhost:{}", port)).underlined());
        }
    }
    
    println!("\n{}", style("Press Ctrl+C to stop all services").dim());
    
    // 終了シグナルを待機
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    
    tokio::spawn(async move {
        let _ = tokio::signal::ctrl_c().await;
        r.store(false, Ordering::SeqCst);
    });
    
    while running.load(Ordering::SeqCst) {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
    
    info!("Stopping services...");
    
    // プロセスをクリーンアップ
    if !options.frontend_only {
        let target_dir = if options.release {
            "target/release/api"
        } else {
            "target/debug/api"
        };
        
        let _ = Command::new("pkill")
            .args(["-f", target_dir])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
    
    if !options.api_only {
        let target_dir = if options.release {
            "target/release/frontend"
        } else {
            "target/debug/frontend"
        };
        
        let _ = Command::new("pkill")
            .args(["-f", target_dir])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
    
    Ok(())
}
