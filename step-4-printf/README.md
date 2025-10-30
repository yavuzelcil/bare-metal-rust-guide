# Step 4: Printf Implementation

Bu adımda `printf` benzeri formatlı çıktı desteği ekliyoruz.

## ✨ Özellikler

- ✅ **`core::fmt::Write`** trait implementation
- ✅ Global **UART instance** (RefCell wrapped)
- ✅ **`print!()`** ve **`println!()`** makroları
- ✅ **Format specifiers:** `{}`, `{:03}`, `{:04X}` vb.
- ✅ Interrupt-safe critical section
- ✅ String interpolation desteği
- ✅ Matematiksel ifade yazdırma

## 🎯 Step 3'ten Farklar

| Özellik | Step 3 (UART) | Step 4 (Printf) |
|---------|---------------|-----------------|
| API | `uart_puts("text")` | **`println!("text")`** |
| Sayı | `print_u32(n)` | **`println!("{}", n)`** |
| Format | Manuel string | **Format specifiers** |
| HEX | ❌ | **`{:X}`, `{:04X}`** |
| Padding | ❌ | **`{:03}`, `{:05}`** |
| Trait | ❌ | **`fmt::Write`** |

## 📝 Kullanım Örnekleri

### Basit Yazdırma

```rust
println!("Hello, World!");
println!();  // Boş satır
print!("No newline");
```

### Değişken Interpolation

```rust
let counter = 42;
println!("Counter: {}", counter);
println!("Multiple: {} and {}", x, y);
```

### Format Specifiers

```rust
// Decimal with padding
println!("Counter: {:03}", 5);     // "Counter: 005"
println!("Value: {:05}", 123);     // "Value: 00123"

// Hexadecimal
println!("Hex: {:X}", 255);        // "Hex: FF"
println!("Hex: 0x{:04X}", 42);     // "Hex: 0x002A"

// Binary (eğer destekleniyorsa)
println!("Binary: {:b}", 7);       // "Binary: 111"
```

### Koşullu Çıktı

```rust
if counter % 5 == 0 {
    println!("  -> Counter is divisible by 5!");
}
```

## 🔧 Nasıl Çalışıyor?

### 1. fmt::Write Trait

```rust
impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.putc(byte);
        }
        Ok(())
    }
}
```

Bu trait, Rust'ın format machinery'sine UART'ı bağlar.

### 2. Global UART Instance

```rust
static mut UART: Option<RefCell<Uart>> = None;

unsafe fn init_global_uart() {
    Uart::init();
    UART = Some(RefCell::new(Uart));
}
```

`RefCell` ile interior mutability sağlıyoruz (runtime borrow checking).

### 3. Critical Section

```rust
struct CriticalSection;

impl CriticalSection {
    fn new() -> Self {
        unsafe {
            core::arch::asm!("cpsid i");  // Interrupt disable
        }
        CriticalSection
    }
}

impl Drop for CriticalSection {
    fn drop(&mut self) {
        unsafe {
            core::arch::asm!("cpsie i");  // Interrupt enable
        }
    }
}
```

RAII pattern ile interrupt-safe erişim.

### 4. println! Macro

```rust
#[macro_export]
macro_rules! println {
    () => { print!("\r\n") };
    ($($arg:tt)*) => {
        with_uart(|uart| {
            use core::fmt::Write;
            let _ = writeln!(uart, $($arg)*);
        })
    };
}
```

Rust'ın `writeln!` makrosunu kullanıyor, `core::fmt` infrastructure'ını kullanır.

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
====================================
  Bare-Metal Rust - Step 4
  Printf Implementation
====================================
Rust version: 0.1.0

Toggle #000 | Total: 1 | Hex: 0x0000
Toggle #001 | Total: 2 | Hex: 0x0001
Toggle #002 | Total: 3 | Hex: 0x0002
Toggle #003 | Total: 4 | Hex: 0x0003
Toggle #004 | Total: 5 | Hex: 0x0004
Toggle #005 | Total: 6 | Hex: 0x0005
  -> Counter is divisible by 5!
...
Toggle #010 | Total: 11 | Hex: 0x000A
  -> Counter is divisible by 5!
  => Reached 10 toggles!
...

=== Summary after 20 toggles ===
  LED state changed 21 times

...
```

## 🎓 Öğrenilen Kavramlar

### 1. Rust Formatting System

`core::fmt` module'ü Rust'ın formatlama altyapısını sağlar:

- **`Display`** trait - user-facing output
- **`Debug`** trait - developer output (`{:?}`)
- **`Write`** trait - output sink (bizim implement ettiğimiz)

### 2. Format Specifiers

```rust
{}       // Display
{:?}     // Debug
{:03}    // Padding (3 digits, leading zeros)
{:X}     // Hexadecimal uppercase
{:x}     // Hexadecimal lowercase
{:b}     // Binary
{:o}     // Octal
```

### 3. Macro Hygiene

`macro_rules!` ile hygenic macro'lar:
- `$($arg:tt)*` - variadic arguments
- Token tree manipulation
- Compile-time code generation

### 4. Interior Mutability

`RefCell<T>` ile runtime borrow checking:
- `borrow_mut()` - mutable borrow
- Panic on multiple mutable borrows
- `no_std` uyumlu

### 5. RAII Pattern

`CriticalSection` ile automatic resource management:
- Constructor'da interrupt disable
- Destructor'da interrupt enable
- Scope-based critical sections

## 📊 Karşılaştırma

| Özellik | Step 3 | Step 4 |
|---------|--------|--------|
| Binary | 66KB | ~**70KB** |
| API Calls | `uart_puts()` | `println!()` |
| Format | Manuel | **Automatic** |
| Type Safety | ❌ | ✅ |
| Ergonomics | Low | **High** |

## ⚙️ İleri Seviye

### Custom Debug Implementation

```rust
struct MyStruct {
    value: u32,
}

impl fmt::Debug for MyStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyStruct {{ value: {} }}", self.value)
    }
}

// Kullanım
let s = MyStruct { value: 42 };
println!("Struct: {:?}", s);
```

### Floating Point (eğer FPU varsa)

```rust
let pi = 3.14159;
println!("Pi: {:.2}", pi);  // "Pi: 3.14"
```

### Error Handling

```rust
fn print_with_error(x: u32) -> fmt::Result {
    with_uart(|uart| {
        write!(uart, "Value: {}", x)
    })
}
```

## 🔜 Sıradaki Adım

**step-5-littlefs** - Embedded dosya sistemi
- Flash üzerinde dosya sistemi
- Dosya okuma/yazma
- Power-safe operations
- Wear leveling

## 📚 Referanslar

- [core::fmt Documentation](https://doc.rust-lang.org/core/fmt/)
- [The Rust Programming Language - Macros](https://doc.rust-lang.org/book/ch19-06-macros.html)
- [RefCell and Interior Mutability](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)

## ⚠️ Troubleshooting

**Compile error: "cannot borrow as mutable"?**
- `RefCell` kullanıyorsanız `borrow_mut()` ile borrow edin
- Critical section içinde çağırın

**Format specifier çalışmıyor?**
- `core::fmt::Write` trait implement edilmiş mi kontrol edin
- `use core::fmt::Write;` import'unu ekleyin

**Binary boyutu çok büyük?**
- Format machinery fazla yer kaplar
- Release build kullanın (`--release`)
- LTO ve optimization ayarlarını kontrol edin

## 💡 İpuçları

- **Buffer:** Büyük string'ler için buffer kullanın
- **Heap:** `no_std` ortamda heap yok, stack kullanın
- **Performance:** Format işlemleri yavaş olabilir, critical path'te dikkatli kullanın
- **Alternative:** Basit durumlar için `uart_puts()` daha hızlıdır
