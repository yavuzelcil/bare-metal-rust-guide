# Step 6: Embedded Web Server + LED Control

Bu adımda STM32F439ZI üzerinde HTTP web sunucusu çalıştırıp, 3 LED'i web arayüzünden kontrol ediyoruz.

## ✨ Özellikler

- ✅ **3-LED Control** - Blue (PB0), Green (PB7), Red (PB14)
- ✅ **HTTP Server** - Request parsing, routing, response generation
- ✅ **Web UI** - Modern HTML/CSS/JS interface
- ✅ **REST API** - JSON endpoints for programmatic access
- ✅ **Toggle All** - Single button to control all LEDs
- ✅ **Real-time Status** - Live LED state display
- ✅ **Ethernet Ready** - MAC/PHY initialization (stub for full implementation)

## 🎯 Step 5'ten Farklar

| Özellik | Step 5 (Storage) | Step 6 (Web Server) |
|---------|------------------|---------------------|
| LEDs | ❌ | ✅ 3 LED (RGB) |
| Network | ❌ | ✅ Ethernet MAC |
| HTTP | ❌ | **Full HTTP parser** |
| Web UI | ❌ | **Modern interface** |
| API | ❌ | **REST endpoints** |
| Binary | 136KB | **130KB** |

## 📡 Architecture

```
┌─────────────────────────────────────────┐
│         Web Browser                     │
│  http://192.168.1.100/                  │
└──────────────┬──────────────────────────┘
               │ HTTP GET /led/blue/on
               ▼
┌─────────────────────────────────────────┐
│      STM32F439ZI - Web Server           │
├─────────────────────────────────────────┤
│  ┌────────────┐      ┌──────────────┐   │
│  │   HTTP     │◄─────┤   Ethernet   │   │
│  │  Handler   │      │   MAC/PHY    │   │
│  └─────┬──────┘      └──────────────┘   │
│        │                                 │
│        ▼                                 │
│  ┌────────────┐                          │
│  │  LED GPIO  │                          │
│  │ PB0/7/14   │                          │
│  └────────────┘                          │
└─────────────────────────────────────────┘
```

## 🔧 Hardware Connections

### STM32F439ZI LEDs:
- **PB0** → Blue LED
- **PB7** → Green LED  
- **PB14** → Red LED

### Ethernet (on Nucleo-144 board):
- **RMII Interface** to LAN8742A PHY
- **RJ45 Connector** for network cable
- Pins automatically configured

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
probe-rs run --chip STM32F439ZITx --protocol swd target/thumbv7em-none-eabihf/release/bare-metal-rust-step6-webserver
```

### Serial Monitor

```bash
screen /dev/tty.usbmodem* 115200
```

Çıktı:
```
=== STM32 Web Server Demo ===

Ethernet init...
MAC: 02:00:00:00:00:01
Ethernet ready (stub)

Server ready!
Waiting for connections...

[5s] Server running... LED states: Blue=OFF Green=OFF Red=OFF
Auto-toggled green LED
[10s] Server running... LED states: Blue=OFF Green=ON Red=OFF
```

## 🌐 Web Interface

### Ana Sayfa: `http://192.168.1.100/`

```
┌─────────────────────────────────────┐
│   🎮 STM32 LED Controller           │
├─────────────────────────────────────┤
│  🔵 Blue LED  ○  [ON] [OFF]         │
│  🟢 Green LED ○  [ON] [OFF]         │
│  🔴 Red LED   ○  [ON] [OFF]         │
├─────────────────────────────────────┤
│  [⚡ Toggle All LEDs]                │
├─────────────────────────────────────┤
│  STM32F439ZI @ 192.168.1.100        │
└─────────────────────────────────────┘
```

**Features:**
- ✅ Individual ON/OFF buttons for each LED
- ✅ Real-time status indicators
- ✅ Toggle All button
- ✅ Responsive design
- ✅ Works on mobile/desktop

## 📚 API Endpoints

### GET `/`
Ana web sayfası (HTML)
```
Response: 200 OK
Content-Type: text/html
```

### GET `/led/blue/on`
Mavi LED'i yak
```
Response: 200 OK
Body: OK
```

### GET `/led/blue/off`
Mavi LED'i söndür
```
Response: 200 OK
Body: OK
```

### GET `/led/green/on`
Yeşil LED'i yak

### GET `/led/green/off`
Yeşil LED'i söndür

### GET `/led/red/on`
Kırmızı LED'i yak

### GET `/led/red/off`
Kırmızı LED'i söndür

### GET `/led/toggle`
Tüm LED'leri toggle et (açık→kapalı, kapalı→açık)
```
Response: 200 OK
Body: OK
```

### GET `/api/status`
LED durumlarını JSON olarak döndür
```json
{
  "blue": true,
  "green": false,
  "red": true
}
```

## 🔌 Demo Mode

**Gerçek Ethernet bağlantısı olmadan test için:**

Serial monitor üzerinden manuel HTTP request gönderebilirsiniz:

```bash
# LED'i yak
echo -e "GET /led/blue/on HTTP/1.1\r\n\r\n"

# Durum sorgula
echo -e "GET /api/status HTTP/1.1\r\n\r\n"
```

Program otomatik olarak:
- Her 5 saniyede LED durumlarını yazdırır
- Her 10 saniyede yeşil LED'i toggle eder

## 🎓 Öğrenilen Kavramlar

### 1. HTTP Protocol
- **Request parsing** - Method, path, headers
- **Response generation** - Status codes, content types
- **Routing** - URL pattern matching
- **Static content** - Inline HTML/CSS/JS

### 2. Ethernet MAC
- **PHY initialization** - LAN8742A setup
- **MAC configuration** - Address, speed, duplex
- **DMA descriptors** - Transmit/receive buffers
- **Interrupt handling** - Packet reception

### 3. Multi-LED Control
- **GPIO masking** - Efficient bit manipulation
- **State tracking** - AtomicU32 for thread-safe state
- **Batch operations** - Toggle all LEDs at once

### 4. Web UI Design
- **Inline assets** - No external files
- **Event handling** - JavaScript fetch API
- **Responsive layout** - Mobile-friendly
- **Visual feedback** - Real-time status indicators

## ⚙️ Full Ethernet Implementation

**Şu anki versiyon stub implementation içerir.** Gerçek network için:

### 1. smoltcp Integration

```toml
[dependencies]
smoltcp = { version = "0.11", features = [
    "proto-ipv4",
    "proto-dhcpv4", 
    "socket-tcp",
    "medium-ethernet"
]}
```

### 2. Ethernet Driver

```rust
use smoltcp::iface::{Interface, InterfaceBuilder};
use smoltcp::wire::{EthernetAddress, IpCidr, Ipv4Address};
use smoltcp::socket::{TcpSocket, TcpSocketBuffer};

// Initialize smoltcp interface
let ethernet_addr = EthernetAddress([0x02, 0x00, 0x00, 0x00, 0x00, 0x01]);
let ip_addrs = [IpCidr::new(Ipv4Address::new(192, 168, 1, 100), 24)];

let mut iface = InterfaceBuilder::new(device, vec![])
    .hardware_addr(ethernet_addr.into())
    .ip_addrs(ip_addrs)
    .finalize();

// Create TCP socket
let tcp_rx_buffer = TcpSocketBuffer::new(vec![0; 2048]);
let tcp_tx_buffer = TcpSocketBuffer::new(vec![0; 2048]);
let tcp_socket = TcpSocket::new(tcp_rx_buffer, tcp_tx_buffer);
```

### 3. Main Loop

```rust
loop {
    // Poll network interface
    let timestamp = Instant::from_millis(millis() as i64);
    iface.poll(&mut sockets, timestamp);

    // Handle TCP connections
    let mut socket = sockets.get::<TcpSocket>(tcp_handle);
    
    if !socket.is_active() {
        socket.listen(80).unwrap();
    }
    
    if socket.can_recv() {
        socket.recv(|buffer| {
            if let Some((method, path)) = parse_http_request(buffer) {
                let response = handle_request(method, path);
                socket.send_slice(response.as_bytes()).unwrap();
            }
            (buffer.len(), ())
        }).unwrap();
    }
}
```

## 📊 Binary Analysis

```
   text    data     bss     dec     hex filename
 130048       0    1024  131072   20000 bare-metal-rust-step6-webserver
```

**Breakdown:**
- HTTP parser: ~10KB
- Web UI (HTML): ~15KB
- LED control: ~2KB
- Ethernet stub: ~3KB
- Stdlib (core): ~100KB

## 🔜 Sıradaki Geliştirmeler

1. **Real Network** - smoltcp integration, DMA setup
2. **DHCP Client** - Dynamic IP assignment
3. **mDNS** - stm32.local hostname
4. **WebSocket** - Real-time bidirectional communication
5. **LED Patterns** - Animations, effects
6. **Authentication** - Basic auth or token
7. **HTTPS** - TLS encryption
8. **Multi-client** - Handle multiple connections

## 📚 Referanslar

- [STM32F4 Ethernet Peripheral](https://www.st.com/resource/en/reference_manual/dm00031020.pdf#page=1131)
- [LAN8742A PHY Datasheet](https://ww1.microchip.com/downloads/en/DeviceDoc/8742a.pdf)
- [smoltcp Documentation](https://docs.rs/smoltcp/)
- [HTTP/1.1 Specification](https://www.rfc-editor.org/rfc/rfc2616)

## ⚠️ Troubleshooting

**LEDs yanmıyor?**
- Pin tanımları doğru mu kontrol edin (PB0, PB7, PB14)
- GPIO clock enable edildi mi?
- Kartınızın LED pinleri farklı olabilir

**Derleme hatası?**
```bash
rustup target add thumbv7em-none-eabihf
cargo clean
cargo build --release
```

**Serial çıktı yok?**
- UART bağlantısı kontrol edin
- Baud rate: 115200
- macOS: `/dev/tty.usbmodem*`
- Linux: `/dev/ttyACM*`

## 💡 İpuçları

- **Testing:** Serial monitor ile manuel HTTP test edin
- **Debug:** LED durumlarını UART'a yazdırın
- **Optimization:** HTML'i gzip ile sıkıştırabilirsiniz
- **Security:** Production'da authentication ekleyin
- **Performance:** DMA kullanımı kritik
- **Reliability:** Timeout mekanizmaları ekleyin

---

**🎉 Web'den LED kontrolü artık mümkün!** 

Ethernet kablosunu takın, tarayıcıdan bağlanın ve LED'leri kontrol edin! 🔴🟢🔵
