# Step 5: Key-Value Storage

Bu adımda embedded sistemler için basit ve etkili bir key-value storage implementasyonu yapıyoruz.

## ✨ Özellikler

- ✅ **Key-Value Storage** (basit NoSQL)
- ✅ RAM-backed flash emulation
- ✅ Set/Get/Delete/List operasyonları
- ✅ Flash-friendly yapı (Entry-based)
- ✅ 16 entry kapasitesi
- ✅ UART üzerinden progress logs

## 🎯 Step 4'ten Farklar

| Özellik | Step 4 (Printf) | Step 5 (Storage) |
|---------|-----------------|------------------|
| Storage | ❌ | ✅ Key-Value store |
| Persistence | ❌ | ✅ (Flash'ta kalıcı) |
| Data API | ❌ | **set/get/delete** |
| External deps | None | **None (pure Rust)** |
| Binary size | 131KB | **136KB (+5KB)** |

## 📁 Key-Value Storage Nedir?

**Key-Value Store**, basit fakat güçlü bir veri saklama yöntemidir:

- **Simple API:** set(key, value), get(key), delete(key)
- **Flash-friendly:** Entry-based structure
- **No external deps:** Pure Rust implementation
- **Small footprint:** Minimal overhead

### Kullanım Alanları

- WiFi credentials (SSID, password)
- Configuration settings (port, timeout)
- Calibration data
- Device parameters
- Runtime preferences

## 🔧 Implementation

### 1. Entry Structure

```rust
#[repr(C)]
struct Entry {
    key: [u8; MAX_KEY_LEN],      // 16 bytes
    key_len: u8,                  // Actual key length
    value: [u8; MAX_VALUE_LEN],  // 64 bytes
    value_len: u8,                // Actual value length
    valid: u8,                    // 0xFF = valid, 0x00 = deleted
}
```

Flash-friendly yapı:
- **Fixed size:** Her entry aynı boyutta (82 bytes)
- **Valid flag:** Delete için flash'ı tekrar yazmaya gerek yok
- **Alignment:** Flash write alignment gereksinimlerine uygun

### 2. Storage Operations

```rust
// Set a key-value pair
storage.set(b"ssid", b"MyWiFiNetwork")?;

// Get a value by key
if let Some(value) = storage.get(b"ssid") {
    println!("SSID: {}", value);
}

// Delete an entry
storage.delete(b"ssid")?;

// List all entries
for (key, value) in storage.list().iter() {
    if let (Some(k), Some(v)) = (key, value) {
        println!("{} = {}", k, v);
    }
}
```

### 3. Demo Flow

1. **Format** - Flash'ı temizle (0xFF ile doldur)
2. **Write** - 4 adet config parametresi yaz
3. **Read** - Parametreleri geri oku
4. **List** - Tüm entry'leri listele
5. **Update** - Bir parametreyi güncelle
6. **Delete** - Bir parametreyi sil
7. **List** - Kalan entry'leri göster

## 🚀 Derleme ve Çalıştırma

### Build

```bash
cargo build --release
```

veya:
```bash
chmod +x build.sh
./build.sh
```

### Flash

```bash
cargo run --release
```

### Serial Monitor

```bash
screen /dev/tty.usbmodem* 115200
```

## 📺 Beklenen Çıktı

```
=== Key-Value Storage Example ===

Formatting storage...
Format complete!

Writing entries...
  ssid = MyWiFiNetwork
  password = secret123
  port = 8080
  timeout = 30

Reading entries...
  ssid = MyWiFiNetwork
  password = secret123
  port = 8080

Listing all entries:
  ssid = MyWiFiNetwork
  password = secret123
  port = 8080
  timeout = 30

Updating 'port' to 9000...
  port = 9000

Deleting 'timeout'...
Remaining entries:
  ssid = MyWiFiNetwork
  password = secret123
  port = 9000

Done!
```

## 📚 Storage Concepts

### Entry-Based Design

Flash memory'ye uygun yapı:
- **Fixed-size entries:** Flash alignment kolay
- **Sequential search:** O(n) ama n=16 küçük
- **Valid flag:** Delete için tekrar yazma gerekmez

### Flash Emulation

Demo için RAM kullanıyoruz:
```rust
static mut FLASH_MEMORY: [u8; 4096] = [0xFF; 4096];
```

Gerçek Flash'ta:
- 4KB = 16 entry × 82 bytes + overhead
- Erase before write
- Wear leveling needed

### Search Strategy

```rust
fn get(&self, key: &[u8]) -> Option<&[u8]> {
    for i in 0..MAX_ENTRIES {
        let entry = self.read_entry(i)?;
        if entry.valid == 0xFF && entry.key == key {
            return Some(&entry.value);
        }
    }
    None
}
```

Linear search - yeterince hızlı çünkü MAX_ENTRIES=16.

## 🎓 Öğrenilen Kavramlar

1. **Data Structures**
   - Fixed-size entries
   - Key-value mapping
   - Linear search
   - Valid flags for deletion

2. **Flash Memory**
   - Erase before write (0xFF pattern)
   - Flash-friendly layouts
   - Alignment requirements
   - Emulation with RAM

3. **Memory Management**
   - Static allocation
   - `#[repr(C)]` for memory layout
   - Slice manipulation
   - Raw pointer usage

4. **Iterator Pattern**
   - Custom iterator implementation
   - Option-based returns
   - Array iteration

## ⚙️ Gerçek Flash Kullanımı

RAM yerine gerçek STM32 Flash kullanmak için:

```rust
use stm32f4xx_hal::flash::{Flash, FlashExt};

const FLASH_START: u32 = 0x080E0000; // Son 128KB

struct Storage {
    flash: Flash,
}

impl Storage {
    fn write_entry(&mut self, index: usize, entry: &Entry) {
        let addr = FLASH_START + (index * size_of::<Entry>()) as u32;
        
        // Erase sector if needed
        self.flash.unlock();
        self.flash.erase_sector(addr);
        
        // Write entry
        let bytes = unsafe {
            core::slice::from_raw_parts(
                entry as *const _ as *const u8,
                size_of::<Entry>(),
            )
        };
        self.flash.program(addr, bytes);
        self.flash.lock();
    }
}
```

**⚠️ Uyarı:** 
- Flash erase slow (~seconds)
- Sector size önemli (128KB sektorlar tipik)
- Write alignment gerekli (word/double-word)

## 📊 Karşılaştırma

| Özellik | Key-Value | Full Filesystem |
|---------|-----------|-----------------|
| Complexity | ⭐ Simple | ⭐⭐⭐ Complex |
| Binary size | +5KB | +30-50KB |
| Dependencies | None | littlefs2/other |
| Use case | Config/params | Files/logs |
| Learning curve | Easy | Moderate |

## 🔜 Sıradaki Adım

**step-6-webserver** - HTTP sunucusu
- TCP/IP stack (embassy-net veya smoltcp)
- HTTP request parsing
- Web UI hosting
- REST API endpoints
- Device'ı web üzerinden kontrol

## 📚 Referanslar

- [STM32 Flash Programming](https://www.st.com/resource/en/programming_manual/pm0081-stm32f40xxx-and-stm32f41xxx-flash-programming-manual-stmicroelectronics.pdf)
- [Embedded Storage Patterns](https://interrupt.memfault.com/blog/data-storage-on-mcu)
- [Flash Wear Leveling](https://www.embedded.com/flash-memory-wear-leveling/)

## ⚠️ Troubleshooting

**Binary boyut arttı?**
- Normal, storage logic eklendi (+5KB)
- `opt-level = "z"` kullanıyoruz
- Release build yapın

**Storage full hatası?**
- MAX_ENTRIES = 16 sınırı
- list() ile dolu entry'leri görün
- delete() ile yer açın

**Get() None dönüyor?**
- Key tam eşleşmeli (case-sensitive)
- valid flag kontrol edin
- format() sonrası set() yapın

## 💡 İpuçları

- **Sizing:** Entry sayısı = Flash size / Entry size
- **Performance:** Linear search yeterince hızlı (n≤16)
- **Reliability:** CRC eklenmesi önerilir
- **Versioning:** Entry structure'a version field ekleyin
- **Migration:** Eski data'yı yeni format'a çevirin
