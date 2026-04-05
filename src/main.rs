use clap::Parser;
use pentester::http_header_analyzer::{analyze_headers, print_report};
use pentester::web_server::start_server;

/// ISU-SecOps-Engine — HTTP Header Analyzer
#[derive(Parser, Debug)]
#[command(name = "pentester", version = "0.1.0", about = "HTTP Güvenlik Header Analiz Aracı")]
struct Args {
    /// Hedef URL (örn: https://example.com)
    #[arg(short, long)]
    url: Option<String>,

    /// JSON formatında çıktı ver
    #[arg(short, long, default_value_t = false)]
    json: bool,

    /// Web panelini başlat
    #[arg(short, long, default_value_t = false)]
    web: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.web {
        start_server().await;
        return;
    }

    match args.url {
        Some(url) => {
            println!("\n🔍 Hedef analiz ediliyor: {}\n", url);
            match analyze_headers(&url).await {
                Ok(report) => {
                    if args.json {
                        println!("{}", serde_json::to_string_pretty(&report).unwrap());
                    } else {
                        print_report(&report);
                    }
                }
                Err(e) => {
                    eprintln!("❌ Hata: {e}");
                    std::process::exit(1);
                }
            }
        }
        None => {
            eprintln!("❌ URL veya --web parametresi gerekli!");
            eprintln!("   Kullanım: cargo run -- --url https://example.com");
            eprintln!("   Panel:    cargo run -- --web");
            std::process::exit(1);
        }
    }
}
