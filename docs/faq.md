# Sık Sorulan Sorular (FAQ)

## Bu araç siteye zarar verir mi?
Hayır. Araç yalnızca normal bir HTTP GET isteği atar ve dönen header'ları okur. Tarayıcınızın yaptığından farklı değildir.

## HTTPS olmayan sitelerde çalışır mı?
Evet, http:// ile başlayan URL'ler de analiz edilebilir. Ancak HTTP sitelerde HSTS zaten eksik olacaktır.

## Hangi sitelerde kullanabilirim?
Yalnızca kendi sitenizde veya test izni aldığınız sistemlerde kullanın. İzinsiz kullanım yasal sorumluluk doğurur.

## JSON çıktısını nasıl kaydedebilirim?
cargo run -- --url https://example.com --json > sonuc.json

## Web paneli dışarıdan erişilebilir mi?
Varsayılan olarak 0.0.0.0:8080 üzerinde çalışır. Güvenlik için yalnızca localhost'ta kullanmanız önerilir.

## Docker ile nasıl çalıştırırım?
docker compose up --build
Tarayıcıda http://localhost:8080 adresine git.

## Testleri nasıl çalıştırırım?
cargo test

