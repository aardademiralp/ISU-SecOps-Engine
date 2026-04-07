# 🔍 Güvenlik Header'ları Detaylı Rehber

## Strict-Transport-Security (HSTS)
Tarayıcıyı her zaman HTTPS kullanmaya zorlar. HTTP üzerinden yapılan istekleri otomatik HTTPS'e yönlendirir.
Eksikse risk: SSL Stripping, HTTP downgrade saldırısı
Önerilen değer: Strict-Transport-Security: max-age=31536000; includeSubDomains

## Content-Security-Policy (CSP)
Hangi kaynaklardan içerik yüklenebileceğini belirler. XSS saldırılarına karşı en etkili savunmadır.
Eksikse risk: XSS (Cross-Site Scripting) saldırısı
Önerilen değer: Content-Security-Policy: default-src 'self'

## X-Frame-Options
Sayfanın iframe içinde gösterilip gösterilemeyeceğini belirler.
Eksikse risk: Clickjacking saldırısı
Önerilen değer: X-Frame-Options: DENY

## X-Content-Type-Options
Tarayıcının MIME türünü tahmin etmesini engeller.
Eksikse risk: MIME sniffing saldırısı
Önerilen değer: X-Content-Type-Options: nosniff

## Referrer-Policy
Sayfa geçişlerinde ne kadar bilgi paylaşılacağını belirler.
Eksikse risk: Hassas URL bilgilerinin sızması
Önerilen değer: Referrer-Policy: strict-origin-when-cross-origin

## Permissions-Policy
Tarayıcı API'larına (kamera, mikrofon, konum) erişimi kısıtlar.
Eksikse risk: Kötü amaçlı API kullanımı
Önerilen değer: Permissions-Policy: camera=(), microphone=(), geolocation=()

## X-XSS-Protection
Eski tarayıcılarda yerleşik XSS filtresini aktif eder.
Eksikse risk: Eski tarayıcılarda XSS saldırısı
Önerilen değer: X-XSS-Protection: 1; mode=block
EOF
