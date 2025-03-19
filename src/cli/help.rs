use console::style;

pub fn print_help() {
    println!("{}", style("Rustorium - Blockchain Platform").bold());
    println!();
    println!("{}", style("USAGE:").yellow());
    println!("  cargo run [OPTIONS]");
    println!();
    println!("{}", style("PORT OPTIONS:").yellow());
    println!("  --api-port <PORT>      API server port (default: auto)");
    println!("  --frontend-port <PORT> Frontend server port (default: auto)");
    println!();
    println!("{}", style("DEVELOPMENT OPTIONS:").yellow());
    println!("  --dev                Start multiple test nodes for development");
    println!("  --nodes <N>          Number of test nodes to start (default: 10)");
    println!("  --base-port <PORT>   Base port for test nodes (default: 40000)");
    println!("  --data-dir <DIR>     Data directory for test nodes");
    println!();
    println!("{}", style("SERVER OPTIONS:").yellow());
    println!("  --api-only          Start API server only");
    println!("  --frontend-only     Start frontend server only");
    println!("  --fast              Fast development mode (lower optimization)");
    println!("  --release           Release mode (higher optimization)");
    println!();
    println!("{}", style("ADDITIONAL OPTIONS:").yellow());
    println!("  --log-level <LEVEL> Set logging level (default: info)");
    println!("  --cors-origin <URL> Set CORS origin (default: *)");
    println!("  --sustainable-demo  Run sustainable blockchain demo");
    println!("  -h, --help         Show this help message");
    println!("  -v, --version      Show version information");
}