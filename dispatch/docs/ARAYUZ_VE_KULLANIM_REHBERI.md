# Dispatch Arayüz ve Kullanım Rehberi

## Genel bakış

Bu rehber, Dispatch arayüzündeki temel bölümleri ve günlük kullanım akışlarını açıklar. Yeni bir workspace açmaktan request oluşturmaya, response incelemekten collection çalıştırmaya kadar uygulamanın başlıca ekranları adım adım ele alınmıştır.

> **Görsel notu:** Bu dokümandaki görsel alanları bilinçli olarak boş bırakılmıştır. Ekran görüntüleri hazırlandığında ilgili alanlara eklenebilir. Görseller için `docs/images/` klasörünün kullanılması önerilir.

## İçindekiler

- [Arayüze genel bakış](#arayüze-genel-bakış)
- [Workspace Launcher](#workspace-launcher)
- [Ana uygulama ekranı](#ana-uygulama-ekranı)
- [Workspace ve üst menü](#workspace-ve-üst-menü)
- [Sol panel](#sol-panel)
- [Request sekmeleri](#request-sekmeleri)
- [Request oluşturma ve gönderme](#request-oluşturma-ve-gönderme)
- [Request yapılandırma alanları](#request-yapılandırma-alanları)
- [Response paneli](#response-paneli)
- [Collection ve folder yönetimi](#collection-ve-folder-yönetimi)
- [Environment yönetimi](#environment-yönetimi)
- [Scriptler ve testler](#scriptler-ve-testler)
- [Collection Runner](#collection-runner)
- [Cookie Manager](#cookie-manager)
- [OpenAPI işlemleri](#openapi-işlemleri)
- [Ayarlar](#ayarlar)
- [Önerilen günlük kullanım akışı](#önerilen-günlük-kullanım-akışı)

## Arayüze genel bakış

Dispatch arayüzü, API ile çalışırken ihtiyaç duyulan bilgilere aynı ekran üzerinden erişilebilecek şekilde düzenlenmiştir. Ana ekran genel olarak aşağıdaki bölümlerden oluşur:

1. **Üst menü:** Workspace işlemleri, aktif environment seçimi, import/export ve uygulama ayarları.
2. **Sol panel:** Collection, request history ve environment yönetimi.
3. **Sekme çubuğu:** Açık request, environment ve runner ekranları arasında geçiş.
4. **Request paneli:** Method, URL, parameter, header, body, authentication ve script ayarları.
5. **Response paneli:** Response body, header, cookie, test ve console sonuçları.

<!-- GÖRSEL ALANI
Dosya önerisi: images/dispatch-ana-ekran.png
İçerik: Dispatch ana ekranının tamamı. Beş ana bölge numaralandırılmalı.
-->

> **Ekran görüntüsü eklenecek:** Dispatch ana ekranı

## Workspace Launcher

Dispatch açıldığında aktif bir workspace bulunmuyorsa **Workspace Launcher** gösterilir. Bu ekran üzerinden yeni bir workspace oluşturulabilir, mevcut bir workspace klasörü açılabilir veya son kullanılan workspace'lerden biri seçilebilir.

### Yeni workspace oluşturma

1. **Yeni workspace oluşturma** seçeneğini kullanın.
2. Workspace için boş veya uygun bir klasör seçin.
3. Workspace adını girin.
4. Oluşturma işlemini onaylayın.

Dispatch seçilen klasörde gerekli workspace belgelerini oluşturur. Collection ve environment verileri bu klasör içinde saklanır.

### Mevcut workspace'i açma

1. **Klasör açma** seçeneğini kullanın.
2. `dispatch.workspace.json` dosyasını içeren klasörü seçin.
3. Web sürümündeyseniz tarayıcının istediği okuma ve yazma iznini onaylayın.

Geçersiz, bozuk veya desteklenmeyen bir workspace seçilirse Dispatch açıklayıcı bir hata gösterir.

### Son kullanılan workspace'ler

Daha önce açılan workspace'ler launcher üzerinde listelenebilir. Bu liste yalnızca hızlı erişim sağlar; workspace verisinin kendisi seçilen klasörde bulunur.

<!-- GÖRSEL ALANI
Dosya önerisi: images/workspace-launcher.png
İçerik: Yeni oluştur, klasör aç ve son kullanılanlar seçeneklerinin göründüğü launcher.
-->

> **Ekran görüntüsü eklenecek:** Workspace Launcher

## Ana uygulama ekranı

Bir workspace açıldığında ana API istemcisi ekranı görüntülenir. Bu ekran, bir request'i hazırlama ve sonucunu inceleme akışını merkezine alır.

Ekranın boyutuna ve kullanıcının tercihine göre sol panelin genişliği ile request/response panellerinin yüksekliği değiştirilebilir. Böylece uzun request gövdeleri veya büyük response'lar üzerinde daha rahat çalışılabilir.

Ana ekranda çalışmaya başlamak için:

1. Yeni bir request sekmesi açın veya collection içinden kayıtlı bir request seçin.
2. HTTP methodunu belirleyin.
3. URL'i girin.
4. Gerekli parameter, header, body ve authentication bilgilerini ekleyin.
5. **Send** butonuna basın.
6. Sonucu response panelinde inceleyin.

<!-- GÖRSEL ALANI
Dosya önerisi: images/ana-ekran-bolumleri.png
İçerik: Sol panel, request alanı ve response alanının ayrı renk veya numaralarla işaretlendiği görünüm.
-->

> **Ekran görüntüsü eklenecek:** Ana ekran bölümleri

## Workspace ve üst menü

Üst menü, açık workspace'e ve uygulamanın genel davranışına ilişkin işlemleri içerir.

Buradan yapılabilen başlıca işlemler şunlardır:

- Açık workspace'in adını görüntüleme
- Başka bir workspace'e geçme
- Masaüstünde workspace'i Backup olarak dışa aktarma
- Masaüstünde workspace'i Safe Share olarak dışa aktarma
- Aktif environment'ı seçme
- OpenAPI içe veya dışa aktarma işlemlerine erişme
- Global HTTP ve görünüm ayarlarını açma

Workspace değiştirilirken kaydedilmemiş request'ler varsa veri kaybını önlemek amacıyla kullanıcıdan onay istenebilir.

<!-- GÖRSEL ALANI
Dosya önerisi: images/ust-menu.png
İçerik: Workspace menüsü, environment seçimi ve ayarlar butonu işaretlenmiş üst bar.
-->

> **Ekran görüntüsü eklenecek:** Üst menü

## Sol panel

Sol panel, workspace içeriğine ve kişisel çalışma geçmişine erişim sağlar. Paneldeki görünüm seçenekleri uygulama sürümüne göre farklı sekme veya butonlarla sunulabilir.

### Collections

Kaydedilmiş collection, folder ve request'ler hiyerarşik bir ağaç yapısında gösterilir. Öğeler genişletilebilir, yeniden adlandırılabilir, çoğaltılabilir, silinebilir veya sürükle-bırak ile yeniden sıralanabilir.

### History

Gönderilmiş request'lerin geçmişi görüntülenir. Bir history kaydı seçildiğinde request bilgileri yeni veya mevcut bir request sekmesinde incelenebilir. History kayıtları silinebilir veya tüm geçmiş temizlenebilir.

### Environments

Workspace'e ait environment'lar listelenir. Bir environment açıldığında değişkenleri ayrı bir sekmede düzenlenebilir.

<!-- GÖRSEL ALANI
Dosya önerisi: images/sol-panel.png
İçerik: Collections, History ve Environments görünümlerini gösteren sol panel.
-->

> **Ekran görüntüsü eklenecek:** Sol panel görünümleri

## Request sekmeleri

Dispatch aynı anda birden fazla çalışma sekmesinin açık tutulmasına izin verir. Sekmeler şu tür içerikleri gösterebilir:

- Yeni veya kaydedilmiş request
- Environment editörü
- Collection Runner

Sekme üzerinde değişiklik yapıldığında kaydedilmemiş durumu belirten bir işaret gösterilebilir. Sekme kapatılırken kaydedilmemiş değişiklikler varsa kullanıcıdan onay istenir.

Kaydedilmiş bir request açıldığında üst bölümde collection ve iç içe folder konumunu gösteren breadcrumb bulunur. Bu bilgi, request'in workspace içindeki yerini anlamayı kolaylaştırır.

<!-- GÖRSEL ALANI
Dosya önerisi: images/request-sekmeleri.png
İçerik: Birden fazla request, environment ve runner sekmesinin açık olduğu görünüm.
-->

> **Ekran görüntüsü eklenecek:** Request ve çalışma sekmeleri

## Request oluşturma ve gönderme

Yeni bir API çağrısı hazırlamak için aşağıdaki adımları izleyin:

1. Yeni bir request sekmesi açın.
2. URL alanının solundan HTTP methodunu seçin.
3. Endpoint adresini URL alanına girin.
4. Gerekirse `Params`, `Headers`, `Body`, `Auth` ve diğer yapılandırma sekmelerini doldurun.
5. Bir environment kullanıyorsanız üst menüden aktif environment'ı seçin.
6. Request'i çalıştırmak için **Send** butonuna basın.
7. Response panelinde status, süre, boyut ve içerik bilgilerini inceleyin.
8. Request'i tekrar kullanacaksanız bir collection içine kaydedin.

URL veya diğer request alanlarında environment değişkenleri kullanılabilir:

```text
{{base_url}}/users/{{user_id}}
```

Çözümlenemeyen bir değişken bulunursa Dispatch request'i göndermeden önce işlemi durdurur ve eksik değişken adlarını gösterir.

<!-- GÖRSEL ALANI
Dosya önerisi: images/request-olusturma.png
İçerik: Method, URL ve Send butonunun numaralandırıldığı request ekranı.
-->

> **Ekran görüntüsü eklenecek:** Request oluşturma alanı

## Request yapılandırma alanları

### Params

Query parameter'lar anahtar-değer satırları olarak tanımlanır. Etkinleştirilen satırlar URL'e query string olarak eklenir. Aynı anahtarın birden fazla kez kullanılması gereken API'ler için tekrar eden parameter'lar eklenebilir.

### Headers

Request header'ları anahtar-değer biçiminde eklenir. Authentication veya body türü tarafından otomatik oluşturulan header'larla çakışmamasına dikkat edilmelidir.

Tarayıcı güvenliği nedeniyle `Cookie` gibi bazı header'lar web sürümünde uygulama tarafından doğrudan ayarlanamaz.

### Body

Request gövdesi için aşağıdaki türlerden biri seçilebilir:

- None
- JSON
- Text
- HTML
- XML
- Form-data
- URL-encoded
- Binary

GET, HEAD ve OPTIONS request'lerinde arayüz body göndermez. JSON editöründeki **Beautify JSON** işlemi yalnızca geçerli JSON içeriklerini biçimlendirir.

### Authentication

Request'e özel authentication yöntemi seçilebilir:

- No Auth
- Bearer Token
- Basic Auth
- API Key
- OAuth 2.0

API Key header veya query parameter olarak uygulanabilir. OAuth seçenekleri kullanılan platforma göre farklılık gösterebilir.

### Request Settings

Request seviyesindeki ayarlar global HTTP ayarlarını geçersiz kılabilir. Override kaldırıldığında ilgili değer tekrar global ayardan alınır.

Masaüstünde HTTP sürümü, SSL doğrulaması ve redirect seçenekleri gibi gelişmiş kontroller bulunur. Web sürümünde tarayıcının yönettiği seçenekler değiştirilemez veya **Tarayıcı yönetir** şeklinde gösterilir.

<!-- GÖRSEL ALANI
Dosya önerisi: images/request-yapilandirma-sekmeleri.png
İçerik: Params, Headers, Body, Auth, Scripts ve Settings sekmelerinin bir arada göründüğü alan.
-->

> **Ekran görüntüsü eklenecek:** Request yapılandırma sekmeleri

## Response paneli

Request tamamlandığında sonuç response panelinde gösterilir. Panelin üst bölümünde genellikle status code, response süresi ve veri boyutu gibi özet bilgiler bulunur.

### Body

Response gövdesi içeriğin türüne göre görüntülenir. Uygun içeriklerde aşağıdaki görünümler kullanılabilir:

- **Pretty:** Okunabilir ve biçimlendirilmiş görünüm
- **Raw:** Response'un ham metin görünümü
- **Preview:** Desteklenen içeriklerin önizlemesi

Binary response'lar metin olarak yorumlanmaz; içerik türüne uygun şekilde ele alınır.

### Headers

Sunucunun döndürdüğü response header'ları listelenir. Content-Type, cache, authentication ve rate-limit bilgileri gibi teknik ayrıntılar buradan incelenebilir.

### Cookies

Masaüstünde response ile alınan cookie'ler Dispatch Cookie Jar'a kaydedilebilir ve ilgili sekmede görüntülenebilir. Web sürümünde cookie görünürlüğü tarayıcının güvenlik kurallarıyla sınırlıdır.

### Tests

Post-response script içinde tanımlanan assertion'ların başarılı veya başarısız sonuçları gösterilir. Her test ayrı bir kayıt olarak incelenebilir.

### Console

Pre-request ve post-response scriptlerinde üretilen `console.log`, `console.info`, `console.warn` ve `console.error` çıktıları görüntülenir.

<!-- GÖRSEL ALANI
Dosya önerisi: images/response-paneli.png
İçerik: Status, süre, Body, Headers, Cookies, Tests ve Console alanları işaretlenmiş response paneli.
-->

> **Ekran görüntüsü eklenecek:** Response paneli

## Collection ve folder yönetimi

Collection'lar, tekrar kullanılacak request'leri proje veya servis bazında düzenlemek için kullanılır.

### Collection oluşturma

1. Sol panelde collection oluşturma seçeneğini kullanın.
2. Açıklayıcı bir collection adı girin.
3. Gerekirse collection içinde folder'lar oluşturun.
4. Request'leri ilgili collection veya folder içine kaydedin.

### Folder kullanımı

Folder'lar servis, kaynak, modül veya kullanım senaryosu bazında düzenlenebilir. Örneğin:

```text
Kullanıcı API
├── Authentication
├── Users
│   ├── Create User
│   ├── Get User
│   └── Update User
└── Roles
```

### Sürükle-bırak işlemleri

Collection ağacındaki folder ve request'ler desteklenen hedeflere sürüklenerek yeniden konumlandırılabilir. Taşıma sonrasında request'in breadcrumb bilgisi yeni konumu gösterir.

### Request kaydetme

Hazırlanan request mevcut bir collection veya folder içine kaydedilebilir. Kayıtlı request üzerinde sonradan yapılan değişikliklerin kalıcı olması için yeniden kaydetme işlemi uygulanmalıdır.

<!-- GÖRSEL ALANI
Dosya önerisi: images/collection-folder-yonetimi.png
İçerik: İç içe folder ve request'lerden oluşan örnek collection ağacı.
-->

> **Ekran görüntüsü eklenecek:** Collection ve folder yapısı

## Environment yönetimi

Environment'lar, aynı request'leri farklı hedef sistemlerde kullanmayı kolaylaştırır.

Örnek environment'lar:

- Local
- Development
- Test
- Staging
- Production

### Environment oluşturma ve düzenleme

1. Sol panelden yeni bir environment oluşturun.
2. Environment'a açıklayıcı bir ad verin.
3. Değişken anahtarlarını ve değerlerini ekleyin.
4. Kullanılmasını istemediğiniz değişkenleri devre dışı bırakın.
5. Değişiklikleri kaydedin.
6. Üst menüden environment'ı aktif hâle getirin.

Örnek değişkenler:

| Anahtar | Örnek değer |
| --- | --- |
| `base_url` | `http://127.0.0.1:8080` |
| `user_id` | `42` |
| `access_token` | Kullanıcıya özel token |

Hassas environment değerlerini ekran görüntüsüne almadan önce gizleyin veya örnek değerlerle değiştirin.

<!-- GÖRSEL ALANI
Dosya önerisi: images/environment-editor.png
İçerik: Environment adı, değişken satırları ve aktif environment seçimi.
-->

> **Ekran görüntüsü eklenecek:** Environment editörü

## Scriptler ve testler

Request üzerinde pre-request ve post-response scriptleri tanımlanabilir.

### Pre-request script

Request gönderilmeden önce çalışır. Environment değerlerini hazırlamak veya request'in method, URL, query, header ve body alanlarını değiştirmek için kullanılabilir.

```javascript
pm.variables.set("trace_id", "dispatch-123");
pm.request.headers.upsert({
  key: "X-Trace-Id",
  value: pm.variables.get("trace_id")
});
```

Pre-request script hata verirse HTTP request gönderilmez.

### Post-response script

Response alındıktan sonra çalışır. Test yazmak, response içinden değer almak veya environment'ı güncellemek için kullanılabilir.

```javascript
const payload = pm.response.json();

pm.test("Status 200", () => {
  pm.response.to.have.status(200);
});

pm.test("Response kimlik içeriyor", () => {
  pm.expect(payload.id).to.exist;
});
```

Test sonuçları **Tests**, console çıktıları ise **Console** sekmesinde gösterilir.

<!-- GÖRSEL ALANI
Dosya önerisi: images/script-editor-ve-testler.png
İçerik: Script editörü ile Tests sonuçlarının birlikte göründüğü ekran.
-->

> **Ekran görüntüsü eklenecek:** Script editörü ve test sonuçları

## Collection Runner

Collection Runner, bir collection içindeki seçili request'leri toplu çalıştırmak için kullanılır. Collection veya uygun folder menüsündeki **Run collection** seçeneğiyle açılır.

### Functional run

Functional modda aşağıdaki seçenekler kullanılabilir:

- Çalıştırılacak request'leri seçme
- Request sırasını değiştirme
- Iteration sayısını belirleme
- Request'ler arası gecikme ekleme
- JSON veya CSV iteration data kullanma
- Hata durumunda çalışmayı durdurma
- Response'ları saklama
- Environment değişikliklerini çalışma sonunda kaydetme

### Performance run

Performance modda süre, virtual-user sayısı ve yük profili belirlenebilir. Çalışma sonunda ortalama, minimum, maksimum, P50, P95, P99 ve request/s gibi metrikler görüntülenir.

### Çalışma sonuçları

Runner ekranında request, test, hata, atlanan adım ve console sonuçları filtrelenebilir. Runner sonuçları yalnızca açık uygulama oturumu boyunca tutulur.

**Stop run**, yeni request planlanmasını durdurur; o anda devam eden HTTP request'i zorla sonlandırmaz.

<!-- GÖRSEL ALANI
Dosya önerisi: images/collection-runner.png
İçerik: Functional veya Performance ayarları ve sonuç tablosu bulunan runner ekranı.
-->

> **Ekran görüntüsü eklenecek:** Collection Runner

## Cookie Manager

Cookie Manager ağırlıklı olarak masaüstü sürümünde kullanılır. Workspace'e ait cookie kayıtları burada görüntülenebilir ve yönetilebilir.

Desteklenen işlemler:

- Domain ekleme
- Cookie oluşturma
- Cookie değerini ve özelliklerini düzenleme
- Cookie'yi etkinleştirme veya devre dışı bırakma
- Tek bir cookie'yi silme
- Workspace Cookie Jar'ını temizleme

Cookie kayıtlarında name, value, domain, path, expiry, Secure, HttpOnly, SameSite, host-only ve enabled gibi alanlar bulunabilir.

Web sürümünde cookie'ler tarayıcı tarafından yönetildiğinden aynı ayrıntılı yönetim ekranı kullanılamaz.

<!-- GÖRSEL ALANI
Dosya önerisi: images/cookie-manager.png
İçerik: Domain listesi, cookie tablosu ve cookie düzenleme formu.
-->

> **Ekran görüntüsü eklenecek:** Cookie Manager

## OpenAPI işlemleri

Dispatch, OpenAPI 3 JSON ve YAML belgelerini içe aktarabilir ve collection'ları OpenAPI 3.0.3 olarak dışa aktarabilir.

### OpenAPI içe aktarma

1. OpenAPI import ekranını açın.
2. Dosya seçin veya belge içeriğini girin.
3. Önizlemede endpoint, folder, server ve security bilgilerini inceleyin.
4. Collection adı ve folder organizasyonunu seçin.
5. Gerekirse server URL'i için environment oluşturun.
6. Uyarıları kontrol edip import işlemini tamamlayın.

### OpenAPI dışa aktarma

1. Dışa aktarılacak collection'ı seçin.
2. Export seçeneklerini ve çözümlenemeyen değişken uyarılarını inceleyin.
3. JSON veya YAML çıktısını oluşturun.

OpenAPI standardı Dispatch scriptlerini ve environment modelinin tamamını temsil etmez. Kayıpsız Dispatch yedeği gerektiğinde masaüstündeki `.dispatch` Backup formatı kullanılmalıdır.

<!-- GÖRSEL ALANI
Dosya önerisi: images/openapi-import-preview.png
İçerik: Endpoint sayısı, folder yapısı, server'lar ve uyarıları gösteren import önizlemesi.
-->

> **Ekran görüntüsü eklenecek:** OpenAPI import önizlemesi

## Ayarlar

Ayarlar ekranı uygulamanın global HTTP davranışını ve görünüm tercihlerini yönetir.

Başlıca ayar grupları:

- **Global:** HTTP sürümü, SSL doğrulaması, redirect ve cookie varsayılanları
- **Proxy:** Desteklenen platformlarda proxy ve proxy authentication ayarları
- **Theme:** Açık, koyu veya sistem temasına uyum seçenekleri

Request seviyesindeki ayarlar, ilgili request için global varsayılanları geçersiz kılabilir. Bir override kaldırıldığında request tekrar global değeri kullanır.

<!-- GÖRSEL ALANI
Dosya önerisi: images/global-settings.png
İçerik: Global, Proxy ve Theme sayfalarının bulunduğu ayarlar penceresi.
-->

> **Ekran görüntüsü eklenecek:** Global ayarlar

## Önerilen günlük kullanım akışı

Dispatch ile düzenli çalışmak için aşağıdaki akış izlenebilir:

1. Çalışılacak proje için doğru workspace'i açın.
2. Hedef sisteme uygun environment'ı seçin.
3. Mevcut collection içinden request'i açın veya yeni request oluşturun.
4. URL, parameter, header, body ve authentication bilgilerini kontrol edin.
5. Request'i gönderin.
6. Status, response body ve header'ları inceleyin.
7. Varsa Tests ve Console sonuçlarını kontrol edin.
8. Gerekli değişiklikleri yaptıktan sonra request'i collection'a kaydedin.
9. İlgili akışın tamamını doğrulamak için Collection Runner kullanın.
10. Workspace paylaşılacaksa hassas değerleri kontrol edin ve mümkünse Safe Share kullanın.

## Ekran görüntüsü hazırlama önerileri

Dokümana eklenecek görsellerin tutarlı olması için:

- Bütün ekran görüntülerini aynı pencere boyutunda alın.
- Mümkünse aynı tema üzerinden ilerleyin.
- Token, parola, cookie ve gerçek kullanıcı verilerini gizleyin.
- Örnek URL ve verilerde yerel veya sahte değerler kullanın.
- Açıklanan bölümleri numara, çerçeve veya ok ile işaretleyin.
- Görselleri `docs/images/` altında kısa ve açıklayıcı adlarla saklayın.
- Görsel alt metinlerini erişilebilir ve açıklayıcı yazın.
- Arayüz değiştiğinde eski ekran görüntülerini de güncelleyin.

Örnek Markdown kullanımı:

```markdown
![Dispatch request oluşturma ekranı](images/request-olusturma.png)
```

## İlgili belgeler

- [Dispatch Nedir?](DISPATCH_NEDIR.md)
- [Ana README](../README.md)
- [Tek codebase mimarisi](UNIFIED_CODEBASE.md)
- [Workspace mimarisi](WORKSPACE_ARCHITECTURE.md)

