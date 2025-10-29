# Step 2: SysTick Timer

Bu adımda ARM Cortex-M SysTick timer'ı kullanarak hassas zamanlama yapıyoruz.

## ✨ Özellikler

- ✅ **SysTick Timer** yapılandırması (1ms tick)
- ✅ **Interrupt Handler** implementasyonu
- ✅ **Atomic** milisaniye sayacı (thread-safe)
- ✅ `millis()` - Mevcut zamanı al
- ✅ `delay_ms()` - Hassas milisaniye delay
- ✅ LED'i **1 saniye** aralıklarla yak/söndür

## 🎯 Step 1'den Farklar

| Özellik | Step 1 (Blinky) | Step 2 (SysTick) |
|---------|-----------------|------------------|
| Zamanlama | Busy-wait loop | **Interrupt tabanlı** |
| Hassaslık | Yaklaşık (~±30%) | **Hassas (±1ms)** |
| CPU Kullanımı | %100 | **Minimal** |
| Delay API | `delay(500_000)` | `delay_ms(1000)` |
| Interrupt | Yok | ✅ SysTick_Handler |
| Vector Table | Minimal (2 entry) | **16 entry** |

## 🔧 SysTick Nedir?

**SysTick** (System Tick Timer), tüm ARM Cortex-M işlemcilerde bulunan basit bir 24-bit down-counter timer'dır:

- Her Cortex-M MCU'da **standart** olarak bulunur
- **0xE000_E010** adresinde (System Control Space)
- İşletim sistemleri için zamanlama (tick) sağlar
- RTOS'larda task switching için kullanılır

### SysTick Register'ları

```
SYST_CSR (0xE000_E010): Control and Status Register
  Bit 0: ENABLE    - Timer'ı etkinleştir
  Bit 1: TICKINT   - Interrupt'ı etkinleştir
  Bit 2: CLKSOURCE - Clock source seç (1=processor clock)
  
SYST_RVR (0xE000_E014): Reload Value Register
  Bit 23-0: RELOAD - Timer'ın reload değeri
  
SYST_CVR (0xE000_E018): Current Value Register
  Bit 23-0: CURRENT - Anlık timer değeri
```

## 📋 Interrupt Vector Table

`memory.x` içinde genişletilmiş vector table:

```ld
.vector_table ORIGIN(FLASH) :
{
  LONG(_estack);           /* 0: Stack pointer */
  LONG(Reset);             /* 1: Reset handler */
  LONG(0);                 /* 2: NMI */
  LONG(0);                 /* 3: HardFault */
  ...
  LONG(SysTick_Handler);   /* 15: SysTick interrupt */
} > FLASH
```

Vector #15 = SysTick interrupt handler

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

LED artık **tam 1 saniye** aralıklarla yanıp sönecek! ⏱️

## 🔍 Kod Detayları

### 1. SysTick Initialization

```rust
unsafe fn systick_init() {
    // 1ms için reload value (16 MHz / 1000 = 16000)
    let reload_value = 16_000 - 1;
    write_volatile(SYST_RVR, reload_value);
    
    // Timer'ı başlat
    write_volatile(SYST_CSR, SYST_CSR_ENABLE | 
                             SYST_CSR_TICKINT | 
                             SYST_CSR_CLKSOURCE);
}
```

### 2. Interrupt Handler

```rust
static MILLIS: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn SysTick_Handler() {
    // Her 1ms'de bir çağrılır
    MILLIS.fetch_add(1, Ordering::Relaxed);
}
```

**Önemli:** `AtomicU32` kullanıyoruz çünkü:
- Interrupt handler ile main loop aynı değişkene erişiyor
- Data race olmaması için atomic operasyon gerekli
- Rust'ın memory safety garantileri

### 3. Delay API

```rust
fn millis() -> u32 {
    MILLIS.load(Ordering::Relaxed)
}

fn delay_ms(ms: u32) {
    let start = millis();
    while millis().wrapping_sub(start) < ms {
        // Busy-wait ama interrupt tabanlı
    }
}
```

### 4. Ana Loop

```rust
loop {
    led_on();
    delay_ms(1000);   // Tam 1 saniye
    
    led_off();
    delay_ms(1000);   // Tam 1 saniye
}
```

## 📊 Zamanlama Hesaplaması

**Sistem Saati:** 16 MHz (HSI - High Speed Internal oscillator)

```
SysTick tick süresi = 1 / System Clock
1ms için gerekli tick = System Clock / 1000

16 MHz için:
Reload Value = (16,000,000 / 1000) - 1 = 15,999
```

**Farklı clock hızları için:**
```rust
// 72 MHz (PLL ile)
let reload_value = 72_000 - 1;

// 168 MHz (STM32F4 max speed)
let reload_value = 168_000 - 1;

// 180 MHz (STM32F439 max speed)
let reload_value = 180_000 - 1;
```

## 🎓 Öğrenilen Kavramlar

1. **Interrupt-Driven Programming**
   - Vector table yapısı
   - Interrupt handler fonksiyonları
   - `#[no_mangle]` ve `extern "C"`

2. **Atomic Operations**
   - `AtomicU32` kullanımı
   - Memory ordering (`Ordering::Relaxed`)
   - Thread-safe shared state

3. **Timer Hardware**
   - SysTick peripheral yapılandırması
   - Reload value hesaplama
   - Interrupt enable/disable

4. **Time Management**
   - Milisaniye sayacı
   - Hassas delay fonksiyonları
   - Overflow-safe subtraction (`wrapping_sub`)

## ⚙️ Konfigürasyon

### Clock Hızını Değiştirme

Sisteminiz farklı clock hızı kullanıyorsa:

```rust
// systick_init() içinde
let system_clock_hz = 16_000_000;  // Sizin clock hızınız
let reload_value = (system_clock_hz / 1000) - 1;
```

### Tick Interval'ini Değiştirme

1ms yerine 100µs (10kHz) için:

```rust
let reload_value = (system_clock_hz / 10_000) - 1;

// Handler'da:
static MICROS: AtomicU32 = AtomicU32::new(0);

fn SysTick_Handler() {
    MICROS.fetch_add(100, Ordering::Relaxed);
}
```

## 🔜 Sıradaki Adım

**step-3-uart** - Seri port haberleşmesi
- UART peripheral yapılandırması
- Character send/receive
- Printf desteği için altyapı
- Debug mesajları yazdırma

## 📚 Referanslar

- [ARM Cortex-M4 Generic User Guide](https://developer.arm.com/documentation/dui0553/latest/)
  - Chapter 4.4: SysTick Timer (SYST)
- [STM32F439 Reference Manual](https://www.st.com/resource/en/reference_manual/dm00031020.pdf)
  - Section 2.2.2: Interrupt Vector Table
- [Rust Atomics and Locks](https://marabos.nl/atomics/)

## ⚠️ Troubleshooting

**LED hala eski hızda yanıp sönüyor?**
1. Flash işlemini tekrar yapın (`cargo run --release`)
2. Kart resetleyin (reset butonu)
3. Clock ayarlarını kontrol edin

**Interrupt çalışmıyor?**
1. Vector table'da `SysTick_Handler` doğru mu?
2. `#[no_mangle]` attribute var mı?
3. Function signature doğru mu: `pub extern "C" fn`

**Zamanlama tutarsız?**
- System clock hızını ölçün/doğrulayın
- Reload value'yu buna göre ayarlayın
- PLL konfigürasyonu yapıldıysa bunu hesaba katın

## 💡 İpuçları

- **Sleep Mode:** Delay sırasında MCU'yu sleep'e alabilirsiniz (enerji tasarrufu)
- **WFI (Wait For Interrupt):** `delay_ms()` içinde kullanılabilir
- **Overflow:** `millis()` ~49 günde overflow olur (32-bit), `wrapping_sub` bunu handle eder
- **Real-Time:** Gerçek RTOS için `cortex-m-rt` crate'ine bakın
