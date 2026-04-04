# ISU-SecOps-Engine — HTTP Header Analyzer

**BGT006 Sızma Testi | İstinye Üniversitesi**  
**Öğrenci:** Aydın Arda Demiralp  
**Branch:** `midterm/aydin-arda-demiralp`

---

## Proje Hakkında

Bu modül, bir hedef web uygulamasının HTTP yanıt header'larını analiz ederek güvenlik açıklarını tespit eden bir Rust aracıdır. Sızma testi sürecinde keşif (reconnaissance) aşamasında kullanılır.

---

## Kontrol Edilen Header'lar

| Header | Eksikse Risk |
|--------|-------------|
| `Strict-Transport-Security` | HTTP downgrade saldırısı |
| `Content-Security-Policy` | XSS saldırısı |
| `X-Frame-Options` | Clickjacking |
| `X-Content-Type-Options` | MIME sniffing |
| `Referrer-Policy` | Bilgi sızıntısı |
| `Permissions-Policy` | Tarayıcı API kötüye kullanımı |
| `X-XSS-Protection` | Eski tarayıcılarda XSS |

---

## Puanlama

| Puan | Not |
|------|-----|
| 90-100 | A |
| 75-89  | B |
| 60-74  | C |
| 45-59  | D |
| < 45   | F |

---

## Kurulum ve Çalıştırma
```bash
cargo build --release
cargo run -- --url https://example.com
cargo run -- --url https://example.com --json
```

---

## Testler
```bash
cargo test
cargo clippy -- -D warnings
cargo fmt
```

---

## Proje Yapısı
```
pentester/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs
    ├── lib.rs
    └── http_header_analyzer/
        ├── mod.rs
        ├── analyzer.rs
        ├── scorer.rs
        └── printer.rs
```
