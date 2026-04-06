# 🤝 Katkıda Bulunma Rehberi

## Geliştirme Ortamı
```bash
git clone https://github.com/aardademiralp/ISU-SecOps-Engine.git
cd ISU-SecOps-Engine
git checkout midterm/aydin-arda-demiralp
cargo build
cargo test
cargo clippy -- -D warnings
cargo fmt
```

## Kod Standartları
- Tüm fonksiyonlara `///` yorum ekleyin
- Hata yönetimi için `Result<>` kullanın
- Her özellik için unit test yazın

## Commit Formatı
```
feat:     Yeni özellik
fix:      Hata düzeltme
docs:     Dokümantasyon
style:    Kod formatı
test:     Test ekleme
```
