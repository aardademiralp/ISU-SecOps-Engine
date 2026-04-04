# 🛡️ ISU-SecOps-Engine — HTTP Header Analyzer

**BGT006 Sızma Testi | İstinye Üniversitesi**

![Rust](https://img.shields.io/badge/Rust-ASYNC-orange?style=for-the-badge&logo=rust)
![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)
![Tests](https://img.shields.io/badge/Tests-13%20passed-brightgreen?style=for-the-badge)
![Clippy](https://img.shields.io/badge/Clippy-clean-blue?style=for-the-badge)

| | |
|---|---|
| **Öğrenci** | Aydın Arda Demiralp |
| **Branch** | `midterm/aydin-arda-demiralp` |
| **Ders** | BGT006 Sızma Testi |
| **Üniversite** | İstinye Üniversitesi |

---

## 📖 Proje Hakkında

**ISU-SecOps-Engine**, sızma testi sürecinin **keşif (reconnaissance)** aşamasında kullanılan bir HTTP güvenlik header analiz aracıdır. Hedef web uygulamasına HTTP isteği atarak dönen header'ları analiz eder, eksik güvenlik başlıklarını tespit eder ve 0–100 arası bir güvenlik puanı (A–F) üretir.

---

## 🚀 Özellikler

- ✅ 7 kritik güvenlik header'ı kontrolü
- ✅ CORS politikası analizi (wildcard tespiti)
- ✅ 0–100 arası güvenlik puanı ve A–F harf notu
- ✅ Renkli terminal çıktısı
- ✅ JSON çıktı desteği (`--json`)
- ✅ Async HTTP istekleri (tokio)
- ✅ 13 unit test

---

## 🔍 Kontrol Edilen Header'lar

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

## 📊 Puanlama Sistemi

| Puan | Not | Anlam |
|------|-----|-------|
| 90–100 | **A** | Mükemmel |
| 75–89 | **B** | İyi |
| 60–74 | **C** | Orta |
| 45–59 | **D** | Zayıf |
| < 45 | **F** | Kritik risk |

> CORS wildcard (`*`) kullanımı −10 puan cezası uygular.

---

## 💻 Örnek Çıktı
```
═══════════════════════════════════════════
🛡  ISU-SecOps-Engine — HTTP Header Analyzer
═══════════════════════════════════════════
🌐 URL         : https://example.com
📡 Durum Kodu  : 200
🖥  Sunucu      : cloudflare

── Güvenlik Header'ları ──────────────────
  ✅  strict-transport-security        HSTS mevcut ✓
  ❌  content-security-policy          CSP eksik — XSS saldırılarına açık
  ❌  x-frame-options                  X-Frame-Options eksik — Clickjacking riski
  ❌  x-content-type-options           X-Content-Type-Options eksik — MIME sniffing riski
  ❌  referrer-policy                  Referrer-Policy eksik — veri sızıntısı riski
  ❌  permissions-policy               Permissions-Policy eksik — tarayıcı API izinleri kontrolsüz
  ❌  x-xss-protection                 X-XSS-Protection eksik (eski tarayıcılar için önerilir)

── CORS Analizi ──────────────────────────
  ℹ️  CORS header'ı yok (API değil, normal)

── Güvenlik Puanı ────────────────────────
  📊 Puan  : 14 / 100
  🏆 Not   : F
═══════════════════════════════════════════
  1 / 7 header geçti
═══════════════════════════════════════════
```

---

## 🛠️ Kurulum ve Çalıştırma

### Gereksinimler
- Rust 1.75+ (`rustup` ile kurulabilir)

### Kurulum
```bash
git clone https://github.com/aardademiralp/ISU-SecOps-Engine.git
cd ISU-SecOps-Engine
cargo build --release
```

### Kullanım
```bash
# Temel kullanım
cargo run -- --url https://example.com

# JSON çıktısı
cargo run -- --url https://example.com --json

# Yardım
cargo run -- --help
```

---

## 🧪 Testler
```bash
# Tüm testleri çalıştır
cargo test

# Clippy kontrolü
cargo clippy -- -D warnings

# Kod formatlama
cargo fmt
```

---

## 🏗️ Proje Yapısı
```
pentester/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs                          # CLI giriş noktası (clap)
    ├── lib.rs                           # Modül tanımları
    └── http_header_analyzer/
        ├── mod.rs                       # Dışa aktarımlar
        ├── analyzer.rs                  # HTTP isteği & header analizi
        ├── scorer.rs                    # Güvenlik puanı hesaplama
        └── printer.rs                   # Renkli terminal çıktısı
```

---

## 📦 Kullanılan Kütüphaneler

| Kütüphane | Amaç |
|-----------|------|
| `tokio` | Async runtime |
| `reqwest` | HTTP istemcisi |
| `clap` | CLI argüman işleme |
| `colored` | Renkli terminal çıktısı |
| `serde_json` | JSON çıktı desteği |
| `url` | URL doğrulama |

---

## ⚖️ Yasal Uyarı

> Bu araç yalnızca **yetkili** sistemlerde **eğitim ve etik sızma testi** amaçlı kullanılmalıdır. İzinsiz kullanım yasal sorumluluk doğurur.	
