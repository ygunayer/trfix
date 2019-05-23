# trfix
Altyazı dosyalarındaki `þ`, `Ð` gibi bozuk Türkçe karakterleri doğrularıyla düzelten program.

> **Not:** Mutlaka [Uyarılar](#uyarılar) bölümünü okuyun!

## Ne, Neden, Nasıl
Günümüzde neredeyse tüm uygulamalar unicode encoding kullansalar da geçmişte birçok uygulama metin dosyalarını açar veya kaydederken çalıştıkları işletim sistemlerindeki varsayılan dile uygun bir encoding kullanmaktaydı. Özellikle Türkçe gibi Latin dillerinde olmayan karakterleri içeren bir dil kullanan bilgisayarlarda hazırlanan dosyalar bir başka dil kullanan bir bilgisayara taşındıklarında bozuk görünebilmekteydi.

Dünya genelinde unicode kullanımı halen %100 seviyesine ulaşmadığından günümüzde bile bu tarz dosyalarla karşılaşabiliyoruz. Ailece izlemek istediğimiz bir dizinin altyazılarında `ÝYÝ SEYÝRLER DÝLERÝZ` yazısını görünce yaşadığım şaşkınlık ve bezginliğin ardından bu uygulamayı yazmaya karar verdim.

## Kurulum ve Çalıştırma
Uygulama henüz bir paketlenmediği için şimdilik elle derleyip kurmanız gerekiyor.

### Derleyerek Kurma
Bilgisayarınızda Rust 2018 Edition ve `cargo`'nun kurulu olduğundan emin olun (benim kullandığım sürüm: 1.33.0, nightly).

Ardından uygulamanın kaynak kodlarının bulunduğu dizine gidin ve aşağıdaki komutları çalıştırın.

```bash
# derle
$ cargo build

# işletim sistemine yükle
$ cargo install --path .
```

Böylece uygulama artık `trfix` komutuyla çalıştırılabilecektir.

### Çalıştırma
Uygulama çalıştırıldığı anda bulunulan dizin ve alt dizinlerindeki `*.srt` veya `*.sub` uzantılı dosyaları işlemeye başlar. Sonu bu uzantıyla biten dizinler, sistem dosyaları ve boyutu 32 MB'tan daha büyük dosyalar işlenmez.

İşlenen her dosya, dosya adının sonuna `.bak` uzantısı eklenerek yedeklenir.

Örnek kullanım:
```bash
$ trfix
```

## TODO
- Travis ile otomatik build ve release süreci
- Metin bazlı olmayan doküman dosyalarını işleyebilme
- Program parametrelerini okuyup çalışma ayarlarını belirleme

## LICENSE
MIT
