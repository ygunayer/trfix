# trfix
`þ`, `Ð` gibi bozuk Türkçe karakterleri doğrularıyla düzelten program.

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
Özel olarak belirtilmediği sürece uygulama çalıştırıldığı dizindeki `*.srt` uzantılı dosyalar üzerinde çalışır. Uygulamayı çalıştırırken vereceğiniz ekstra bir parametreyle bunu dilediğiniz bir [glob](https://docs.python.org/3/library/glob.html) pattern'ı ile değiştirebilirsiniz.

Örnek kullanımlar:
```bash
# bulunulan dizindeki .srt uzantılı dosyaları değiştir
$ trfix

# bulunulan dizin ve tüm alt dizinlerindeki .srt uzantılı dosyaları değiştir
$ trfix **/*.srt

# bilgisayardaki TÜM .srt uzantılı dosyaları değiştir (TAVSİYE EDİLMEZ)
$ trfix /**/*.srt
```

## Uyarılar
Bu uygulama halen yapım aşamasındadır ve başta veri kaybı olmak üzere çeşitli sorunlara yol açabilir. Kullanmadan önce verilerinizi yedeklemeyi unutmayın ve kullanım esnasında gerçekten ihtiyacınız olan dosyaları değiştirmeye çalıştığınızdan emin olun.

Öngörülen birtakım problemler aşağıda listelenmiştir:

- Uygulama orijinal dosyayı güncellemek yerine yeni bir dosya oluşturup (orijinal dosya adının sonuna `.tmp` ekleyerek) yeni içerikle doldurmakta ve ardından orijinal dosyayı silip, yeni dosyanın adını orijinaliyle değiştirmektedir. Bu da disk alanının yetersiz olması, bir şekilde dosya adı değiştirilememesi, yedek dosyayla aynı isimde bir dosyanın zaten var olması, v.b. durumlarda veri kaybı oluşmasına neden olabilir
- Uygulamanın performanslı çalışması gözetilmemiştir ve dosyları satır satır incelemektedir. Metin içermeyen ya da hiç satır içermeyen milyonlarca karakterlik bir dosya işlenmeye çalışıldığı takdirde sistemi kararsız hale getirebilir
- Önemli sistem dosyalarını da kapsayacak bir `glob` pattern verilmesi, muazzam veri kaybının yanısıra, işletim sistemini veya kurulu diğer uygulamaları geri dönüşü olmayacak şekilde bozabilir
- Binary ya da düz metin dosyaları arasında herhangi bir ayrım yapılmadığı için farklı dosya tiplerinde veri kayıpları yaratabilir; dosyaları bozabilir
    - Buna Word, Excel, PDF gibi metin içeren fakat tanım gereği *binary* formatta olan dosyalar dahildir. Yani `trfix *.doc` gibi bir komut büyük ihtimalle iyi sonuçlanmayacaktır

## TODO
- Gerçek Unicode kontrolü
- Doğru hata ve akış kontrolü
- Dosyaların tiplerini tespit edip, işlenemeyecek olanları reddetme
- Veri kaybını minimuma indirmek
- Çeşitli flag'ler yoluyla kullanıcıya daha fazla kontrol verme
    - `glob` yerine daha belli dosya uzantılarına izin verip recursion için özel flag eklemek olabilir

## LICENSE
MIT
