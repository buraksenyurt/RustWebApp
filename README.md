# Rust Web App

Rust programlama dili kullanılarak web uygulaması geliştirme sürecinin tecrübe edildiği repodur. Kaynak olarak Packt yayınlarınn [Rust Web Programming: A hands-on guide to Rust for modern web development, with microservices and nanoservices , Third Edition ](https://www.packtpub.com/en-us/product/rust-web-programming-9781835887769) isimli kitabından yararlanılmaktadır.

## Senaryo

Senaryo da planlı işler ele alınır. Her bir planlı işin kısa bir başlığı, Scrum yaklaşımındakine benzer Fibonacci temelli büyüklüğü _(1,2,3,5,8,13 gibi)_ ve durum bilgisi _(Ready, In-Progress, Completed)_ vardır.

## Proje Yapısı

Günden günde gelişen proje yapısı aşağıdaki ağaç yapısı ile ifade edilebilir.

- **Root**
  - **Core** (Binary. CLI ile Work Item ekleme, silme, listeleme gibi özellikler de içerir, api desteği sunar)
  - **Dal** (Library. Data Access Layer görevini üstlenir)
  - **Server** (Binary. Actix-Web framework'ü kullanan, asenkron operasyonlarda Tokio küfesi ile çalışan web server)

## Day_00

CLI komutları ile Work Item'lar eklenebilmelidir. Ayrıca web server başlatılabilmeli ve localhost:3000 adresine erişildiğinde index sayfası gelmelidir.

```bash
#core programında
cargo run -- create -t "Study 25 minutes for Rust" -v 5 -s "completed"
cargo run -- create -t "Run for 10 Km in 2 hour" -v 8 -s "Ready"

#Web Server'ı çalıştırmak için
# root klasörde
cargo run -p server
```

## Day_01

