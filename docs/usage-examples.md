# Kullanım Örnekleri

## Temel Kullanım

### Tek URL analizi
cargo run -- --url https://example.com

### JSON çıktısı
cargo run -- --url https://example.com --json

### JSON dosyaya kaydet
cargo run -- --url https://example.com --json > sonuc.json

### Web panel
cargo run -- --web

## Docker ile Kullanım

### Image oluştur ve çalıştır
docker compose up --build

### CLI olarak kullan
docker run isu-secops-engine --url https://example.com

### Web panel
docker compose up
Tarayıcıda http://localhost:8080 aç

## Örnek Senaryolar

### E-ticaret sitesi testi
cargo run -- --url https://hedef-eticaret.com

### Kurumsal uygulama testi
cargo run -- --url https://kurumsal-uygulama.com --json

### API guvenlik kontrolu
cargo run -- --url https://api.hedef.com
