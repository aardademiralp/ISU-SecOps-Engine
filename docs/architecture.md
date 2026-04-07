# 🏗️ Mimari

ISU-SecOps-Engine, modüler bir Rust projesi olarak tasarlanmıştır.

## Genel Akış
Kullanıcı → CLI/Web Panel → Analyzer → Scorer → Printer

## Modüller

### `main.rs`
CLI giriş noktasıdır. `clap` kütüphanesi ile argümanları işler.
`--url`, `--json`, `--web` flaglerini karşılar.

### `analyzer.rs`
Hedef URL'ye async HTTP GET isteği atar.
Dönen header'ları okur ve eksikleri tespit eder.

### `scorer.rs`
Her header için puan hesaplar.
CORS wildcard tespiti yapar ve −10 ceza uygular.
0–100 arası puan ve A–F notu üretir.

### `printer.rs`
Renkli terminal çıktısı üretir.
JSON modu aktifse `serde_json` ile çıktı verir.

## Teknoloji Seçimleri

| Teknoloji | Neden? |
|---|---|
| Rust | Bellek güvenliği, yüksek performans |
| tokio | Async HTTP istekleri |
| reqwest | Güvenilir HTTP istemcisi |
| axum | Hafif ve hızlı web sunucusu |
| clap | Güçlü CLI argüman işleme |
