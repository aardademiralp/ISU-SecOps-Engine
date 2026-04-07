# 🛡️ ISU-SecOps-Engine — HTTP Header Analyzer

![İstinye logo](https://www.istinye.edu.tr/sites/default/files/2025-07/isu_logo_tr-1.svg)
**BGT006 Sızma Testi | İstinye Üniversitesi**

![Rust](https://img.shields.io/badge/Rust-ASYNC-orange?style=for-the-badge&logo=rust)
![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)
![Tests](https://img.shields.io/badge/Tests-13%20passed-brightgreen?style=for-the-badge)
![Clippy](https://img.shields.io/badge/Clippy-clean-blue?style=for-the-badge)
![Build Status](https://img.shields.io/badge/build-passing-brightgreen)

| | |
|---|---|
| **Öğrenci** | Aydın Arda Demiralp |
| **Branch** | `midterm/aydin-arda-demiralp` |
| **Ders** | BGT006 Sızma Testi |
| **Üniversite** | İstinye Üniversitesi |
| **Danışman / Instructor** | Keyvan Arasteh |

---

## Table of Contents

- [📖 Proje Hakkında](#-proje-hakkında)
- [🚀 Özellikler](#-özellikler)
- [🔍 Kontrol Edilen Header'lar](#-kontrol-edilen-headerlar)
- [📊 Puanlama Sistemi](#-puanlama-sistemi)
- [💻 Örnek Çıktı](#-örnek-çıktı)
- [🛠️ Kurulum ve Çalıştırma](#️-kurulum-ve-çalıştırma)
- [🧪 Testler](#-testler)
- [🏗️ Proje Yapısı](#️-proje-yapısı)
- [📦 Kullanılan Kütüphaneler](#-kullanılan-kütüphaneler)
- [⚖️ Yasal Uyarı](#️-yasal-uyarı)
- [🎯 Gerçek Dünya Senaryoları](#-gerçek-dünya-senaryoları)
- [❓ Sık Sorulan Sorular (FAQ)](#-sık-sorulan-sorular-faq)
- [🤝 Katkıda Bulunma](#-katkıda-bulunma)
- [📝 Versiyon Geçmişi](#-versiyon-geçmişi)
---

## 📖 Proje Hakkında

**ISU-SecOps-Engine**, sızma testi sürecinin **keşif (reconnaissance)** aşamasında kullanılan bir HTTP güvenlik header analiz aracıdır. Hedef web uygulamasına HTTP isteği atarak dönen header'ları analiz eder, eksik güvenlik başlıklarını tespit eder ve 0–100 arası bir güvenlik puanı (A–F) üretir.

---
## 🎥 Proje Demosu


<video width="100%" controls>
  <source src="https://raw.githubusercontent.com/aardademiralp/ISU-SecOps-Engine/midterm/aydin-arda-demiralp/demo/demo.mp4" type="video/mp4">
</video>

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
### Web Panel (YENİ!)
```bash
# Web panelini başlat
cargo run -- --web
> Tarayıcı otomatik olarak açılır. URL girerken `https://` yazmana gerek yok, otomatik eklenir.

### Web Panel Özellikleri
- 🎨 Karanlık/Aydınlık tema
- 📊 Progress bar (geçen/toplam header)
- 🕓 Analiz geçmişi (son 5 analiz)
- 📋 JSON çıktı görüntüleme
- 📄 Sonuçları kopyalama
- 🌍 Türkiye saat/tarih formatı```

Tarayıcıdan `http://localhost:8080` adresine git.
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
---

## 🎯 Gerçek Dünya Senaryoları

### Senaryo 1: E-ticaret Sitesi Testi
Bir e-ticaret sitesini test ettiğinizde CSP eksikse, saldırgan ödeme sayfasına zararlı JavaScript enjekte ederek kredi kartı bilgilerini çalabilir.
```bash
cargo run -- --url https://hedef-eticaret.com
```

### Senaryo 2: Kurumsal Web Uygulaması
HSTS eksik bir kurumsal uygulamada, şirket ağındaki saldırgan SSL Stripping yaparak kullanıcı oturumlarını ele geçirebilir.
```bash
cargo run -- --url https://kurumsal-uygulama.com --json
```

### Senaryo 3: API Güvenlik Kontrolü
Bir REST API'nin CORS yapılandırmasını kontrol ederken wildcard (*) kullanımı tespit edilirse, herhangi bir siteden API'ya istek atılabilir demektir.
```bash
cargo run -- --url https://api.hedef.com
```

---

## ❓ Sık Sorulan Sorular (FAQ)

**S: Bu araç siteye zarar verir mi?**
H: Hayır. Araç yalnızca normal bir HTTP GET isteği atar ve dönen header'ları okur. Tarayıcınızın yaptığından farklı değildir.

**S: HTTPS olmayan sitelerde çalışır mı?**
H: Evet, `http://` ile başlayan URL'ler de analiz edilebilir. Ancak HTTP sitelerde HSTS zaten eksik olacaktır.

**S: Hangi sitelerde kullanabilirim?**
H: Yalnızca kendi sitenizde veya test izni aldığınız sistemlerde kullanın. İzinsiz kullanım yasal sorumluluk doğurur.

**S: JSON çıktısını nasıl kaydedebilirim?**
H: 
```bash
cargo run -- --url https://example.com --json > sonuc.json
```

**S: Web paneli dışarıdan erişilebilir mi?**
H: Varsayılan olarak `0.0.0.0:8080` üzerinde çalışır. Güvenlik için yalnızca `localhost`'ta kullanmanız önerilir.

---

## 🤝 Katkıda Bulunma

Projeye katkıda bulunmak için:

1. Repoyu fork edin
2. Feature branch oluşturun: `git checkout -b feat/yeni-ozellik`
3. Değişikliklerinizi commit edin: `git commit -m "feat: yeni özellik eklendi"`
4. Branch'i push edin: `git push origin feat/yeni-ozellik`
5. Pull Request açın

### Commit Mesajı Formatı
```
feat:     Yeni özellik
fix:      Hata düzeltme
docs:     Dokümantasyon
style:    Kod formatı
refactor: Kod yeniden yapılandırma
test:     Test ekleme
```

---

## 📝 Versiyon Geçmişi

### v0.1.0 (2026-04-05)
- ✅ 10 güvenlik header kontrolü
- ✅ CORS analizi
- ✅ 0-100 puanlama sistemi (A-F)
- ✅ Renkli terminal çıktısı
- ✅ Web panel (axum + HTML/JS)
- ✅ JSON çıktı desteği
- ✅ 13 unit test
- ✅ GPL-3.0 lisansı
