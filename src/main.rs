use clap::Parser;
use pentester::http_header_analyzer::{analyze_headers, print_report};

/// ISU-SecOps-Engine — HTTP Header Analyzer
#[derive(Parser, Debug)]
#[command(
    name = "pentester",
    version = "0.1.0",
    about = "HTTP Güvenlik Header Analiz Aracı"
)]
struct Args {
    /// Hedef URL (örn: https://example.com)
    #[arg(short, long)]
    url: String,

    /// JSON formatında çıktı ver
    #[arg(short, long, default_value_t = false)]
    json: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("\n🔍 Hedef analiz ediliyor: {}\n", args.url);

    match analyze_headers(&args.url).await {
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
