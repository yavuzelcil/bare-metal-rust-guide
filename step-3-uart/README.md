# Step 3: UART Serial Haberleşme

Bu adımda UART üzerinden seri port haberleşmesi yapıyoruz.

## Hedefler

- UART register'larını yapılandırma
- Baud rate ayarları
- Karakter gönderme ve alma
- Basit echo programı

## Kullanım

```bash
cd step-3-uart
cargo build --release
cargo run --release
```

Serial terminal ile bağlanmak için:
```bash
screen /dev/tty.usbmodem* 115200
# veya
minicom -D /dev/tty.usbmodem* -b 115200
```

---

**Not:** Bu dizin henüz tamamlanmadı. İçerik gelecekte eklenecek.
