# Step 6: Embedded Web Server

Bu adımda basit bir HTTP web sunucusu çalıştırıyoruz.

## Hedefler

- Ethernet veya WiFi üzerinden TCP/IP
- HTTP request parsing
- Static web sayfası sunma
- REST API endpoint'leri
- Embedded device'ı web üzerinden kontrol

## Kullanım

```bash
cd step-6-webserver
cargo build --release
cargo run --release
```

Web arayüzüne erişim:
```
http://<device-ip>/
```

---

**Not:** Bu dizin henüz tamamlanmadı. İçerik gelecekte eklenecek.
