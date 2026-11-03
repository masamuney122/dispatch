# Dispatch Nedir?

## Genel bakış

Dispatch; HTTP isteklerini oluşturmak, göndermek, incelemek, test etmek ve düzenli bir şekilde saklamak için geliştirilen **local-first bir API istemcisidir**. Bir API'nin geliştirilmesi veya doğrulanması sırasında ihtiyaç duyulan request oluşturma, authentication, environment yönetimi, otomatik test, toplu çalıştırma ve dokümantasyon aktarımı gibi işlemleri tek bir çalışma alanında bir araya getirir.

Dispatch hem **Tauri tabanlı masaüstü uygulaması** hem de **Chromium tabanlı web/PWA uygulaması** olarak çalışabilir. İki sürüm aynı React ve TypeScript arayüzünü, aynı veri modellerini ve mümkün olan yerlerde aynı Rust tabanlı iş kurallarını kullanır. Bu nedenle kullanıcı deneyimi ve workspace yapısı platformlar arasında büyük ölçüde ortaktır.

> Dispatch aktif geliştirme aşamasındadır. Workspace veri formatı sürümlü olmakla birlikte henüz kararlı bir genel sürüm ve tüm gelecek sürümler için geriye dönük uyumluluk garantisi verilmemektedir.

## İçindekiler

- [Dispatch hangi problemi çözer?](#dispatch-hangi-problemi-çözer)
- [Kimler için tasarlanmıştır?](#kimler-için-tasarlanmıştır)
- [Temel yaklaşım](#temel-yaklaşım)
- [Temel fonksiyonlar](#temel-fonksiyonlar)
- [Dispatch nasıl çalışır?](#dispatch-nasıl-çalışır)
- [Web ve masaüstü sürümleri](#web-ve-masaüstü-sürümleri)
- [Veri saklama ve gizlilik yaklaşımı](#veri-saklama-ve-gizlilik-yaklaşımı)
- [Örnek kullanım senaryoları](#örnek-kullanım-senaryoları)
- [Mevcut sınırlar](#mevcut-sınırlar)
- [Temel kavramlar](#temel-kavramlar)

## Dispatch hangi problemi çözer?

API geliştirme sürecinde bir endpoint'i yalnızca çağırmak çoğu zaman yeterli değildir. Geliştiricinin farklı HTTP methodlarını ve body türlerini denemesi, kimlik doğrulama bilgilerini yönetmesi, geliştirme ve canlı ortamlar arasında geçiş yapması, response içeriğini incelemesi ve aynı testleri tekrar çalıştırması gerekir. Takım içinde kullanılan request'lerin düzenli ve taşınabilir biçimde saklanması da sürecin önemli bir parçasıdır.

Dispatch bu ihtiyaçları aşağıdaki başlıklarda birleştirir:

- HTTP request oluşturma ve gönderme
- Response gövdesi, header, cookie, süre ve durum bilgisini inceleme
- Request'leri collection ve folder yapısında düzenleme
- Farklı ortamlar için değişken tanımlama
- Authentication bilgilerini request'e uygulama
- Request öncesinde ve response sonrasında JavaScript çalıştırma
- Response sonuçları üzerinde otomatik assertion'lar oluşturma
- Bir collection içindeki request'leri toplu veya performans odaklı çalıştırma
- OpenAPI belgelerini içe ve dışa aktarma
- Workspace verisini yerel, taşınabilir ve sürümlü bir formatta saklama

Bu yapı sayesinde tek seferlik denemeler ile tekrar kullanılabilir API test senaryoları aynı uygulama içinde yönetilebilir.

## Kimler için tasarlanmıştır?

Dispatch başta yazılım geliştirme ekipleri olmak üzere API ile çalışan farklı rollere hitap eder:

### Backend geliştiricileri

Endpoint'leri geliştirme sırasında çağırabilir, farklı request gövdelerini deneyebilir, authentication akışlarını doğrulayabilir ve response içeriği için testler yazabilir.

### Frontend ve mobil geliştiricileri

Entegre olacakları servisleri uygulamadan bağımsız olarak inceleyebilir, örnek response'ları görebilir ve farklı hata durumlarını test edebilir.

### Test ve kalite ekipleri

Tekrarlanabilir request senaryoları hazırlayabilir, assertion'lar tanımlayabilir ve collection'ları farklı veri setleriyle çalıştırabilir.

### Teknik ekip liderleri ve proje ekipleri

Bir projeye ait endpoint'leri collection ve folder yapısında düzenleyebilir, ekip içinde paylaşılabilir workspace'ler oluşturabilir ve API kapsamını daha görünür hâle getirebilir.

### API'yi öğrenen veya keşfeden kullanıcılar

OpenAPI belgesini içe aktararak endpoint'leri hazır bir collection olarak inceleyebilir ve request'leri doğrudan deneyebilir.

## Temel yaklaşım

### Local-first çalışma

Dispatch'te workspace verisinin ana kaynağı kullanıcının seçtiği yerel klasördür. Collection'lar, kaydedilmiş request'ler ve environment'lar bu klasörde saklanır. Temel çalışma akışı merkezi bir bulut hesabına bağlı değildir.

Local-first yaklaşımın sağladığı başlıca faydalar şunlardır:

- Kullanıcı verinin nerede saklandığını bilir.
- Workspace normal dosya yedekleme ve sürümleme süreçlerine dahil edilebilir.
- Proje verisi uygulama kurulumundan bağımsız taşınabilir.
- Aynı workspace formatı web ve masaüstü sürümlerinde kullanılabilir.
- Kişisel history ve arayüz tercihleri paylaşılan proje verisinden ayrı tutulur.

Local-first, bütün verilerin güvenliğinin otomatik olarak sağlandığı anlamına gelmez. Workspace içinde token, parola veya başka hassas değerler tutuluyorsa klasörün paylaşımı ve erişim izinleri kullanıcı tarafından dikkatle yönetilmelidir.

### Tek codebase, iki platform

Dispatch'in web ve masaüstü sürümleri birbirinden bağımsız iki ürün olarak geliştirilmez. Ortak arayüz ve iş akışı korunur; HTTP motoru, dosya sistemi ve cookie yönetimi gibi platforma bağlı işlemler uygun adapter tarafından gerçekleştirilir.

Bu yaklaşımın amacı:

- İki platformda tutarlı bir kullanıcı deneyimi sağlamak
- Aynı iş kuralının farklı sürümlerde farklı davranmasını önlemek
- Workspace'lerin platformlar arasında taşınabilmesini sağlamak
- Yeni fonksiyonların ortak kod üzerinden geliştirilebilmesini kolaylaştırmaktır

### Workspace odaklı organizasyon

Dispatch içindeki temel çalışma alanı **workspace**'tir. Bir workspace belirli bir proje, servis grubu veya API kapsamına ait collection ve environment verilerini bir arada tutar.

Örnek bir organizasyon:

```text
E-Ticaret API Workspace
├── Kullanıcı İşlemleri
│   ├── Kullanıcı oluştur
│   ├── Kullanıcı getir
│   └── Kullanıcı güncelle
├── Sipariş İşlemleri
│   ├── Sipariş oluştur
│   └── Sipariş durumu getir
└── Ödeme İşlemleri
    ├── Ödeme başlat
    └── Ödeme durumunu sorgula
```

## Temel fonksiyonlar

### 1. HTTP request oluşturma ve gönderme

Dispatch aşağıdaki HTTP methodlarını destekler:

- GET
- POST
- PUT
- PATCH
- DELETE
- HEAD
- OPTIONS

Bir request üzerinde URL, query parameter, header, body, authentication ve request'e özel HTTP ayarları tanımlanabilir. Desteklenen body türleri arasında JSON, düz metin, HTML, XML, form-data, URL-encoded form ve binary dosya bulunur.

Request gönderildikten sonra kullanıcı response body, status code, header, cookie, test sonucu, console çıktısı ve response süresi gibi bilgileri inceleyebilir.

### 2. Collection ve folder yönetimi

Tekrar kullanılacak request'ler collection içine kaydedilebilir. Collection'lar iç içe folder'lar içerebilir; böylece geniş API'ler servis, modül, kaynak veya iş akışına göre düzenlenebilir.

Bu yapı aşağıdaki amaçlarla kullanılabilir:

- Aynı projeye ait request'leri gruplamak
- Belirli bir kullanıcı akışını sıralı şekilde göstermek
- Test senaryolarını ekip içinde anlaşılır hâle getirmek
- Collection Runner için çalıştırılabilir request setleri hazırlamak

### 3. Environment ve değişken yönetimi

Local, test, staging ve production gibi farklı ortamlar için ayrı environment'lar tanımlanabilir. Request alanlarında çift süslü parantez söz dizimi kullanılır:

```text
{{base_url}}/users/{{user_id}}
```

Değişkenler URL, query parameter, header, metin tabanlı body, form alanları ve authentication bilgileri içinde kullanılabilir. Aktif environment değiştirildiğinde aynı request farklı bir hedefe veya farklı kimlik bilgilerine uyarlanabilir.

Eksik bir değişken varsa Dispatch request'i göndermeden önce işlemi durdurur ve eksik değişkenleri kullanıcıya bildirir. Böylece çözülmemiş değerlerle yanlış bir çağrı yapılmasının önüne geçilir.

### 4. Authentication desteği

Dispatch yaygın API authentication yöntemlerini destekler:

- Bearer Token
- Basic Authentication
- Header veya query parameter üzerinden API Key
- OAuth 2.0 Client Credentials
- OAuth 2.0 Password Grant
- OAuth 2.0 Authorization Code ve PKCE

Platform güvenlik modeli nedeniyle bazı OAuth akışlarının kullanılabilirliği web ve masaüstü sürümlerinde farklıdır. Authorization Code ve yerel callback listener gerektiren tam akış masaüstünde desteklenir.

### 5. Pre-request ve post-response scriptleri

Kaydedilmiş bir request için iki farklı JavaScript çalışma aşaması tanımlanabilir:

- **Pre-request script:** Request gönderilmeden önce çalışır. URL, method, header, query veya body üzerinde değişiklik yapabilir ve environment değerlerini güncelleyebilir.
- **Post-response script:** Response alındıktan sonra çalışır. Response'u inceleyebilir, test assertion'ları üretebilir, environment değerlerini güncelleyebilir ve console çıktısı oluşturabilir.

Script ortamı, Postman'dan geçişi kolaylaştırmak amacıyla belgelenmiş bir `pm.*` API alt kümesi sunar. Dispatch'in `dp.*` alias'ı da geriye uyumluluk için desteklenir.

Scriptler ana uygulamadan izole edilmiş QuickJS ortamında ve ayrı bir Web Worker içinde çalıştırılır. Çalışma süresi, bellek ve erişilebilir host API'leri sınırlandırılmıştır.

### 6. Otomatik testler ve assertion'lar

Post-response script içinde status code, header veya response body üzerinde assertion tanımlanabilir. Her assertion sonucu ayrı ayrı raporlanır.

Örnek:

```javascript
pm.test("Status kodu 200 olmalı", () => {
  pm.response.to.have.status(200);
});

pm.test("Response bir kullanıcı kimliği içermeli", () => {
  const body = pm.response.json();
  pm.expect(body.id).to.exist;
});
```

Bu özellik manuel API kontrolünü tekrarlanabilir bir doğrulama senaryosuna dönüştürür.

### 7. Collection Runner

Collection Runner, bir collection içindeki seçili request'leri belirlenen sırayla çalıştırır. İki temel çalışma modu bulunur:

- **Functional run:** Request ve test akışlarının doğrulanmasına odaklanır. Iteration sayısı, request'ler arası gecikme, JSON/CSV test verisi ve hata durumunda durdurma gibi seçenekler sunar.
- **Performance run:** Belirli bir süre ve virtual-user sayısı altında yük oluşturur. Ortalama, minimum, maksimum, P50, P95, P99 ve request/s gibi metrikler üretir.

Runner; normal request gönderimiyle aynı environment çözümleme, script ve test akışını kullanır. Böylece tek bir request üzerinde çalışan senaryo toplu çalıştırmada da tutarlı davranır.

### 8. Request history

Gönderilen request'ler aktif workspace ile ilişkilendirilmiş history içinde tutulur. History, daha önce yapılan çağrıların yeniden incelenmesini ve tekrarlanmasını kolaylaştırır.

History cihaz ve uygulama oturumuna ait bir veri olarak değerlendirilir; paylaşılan workspace'in collection ve environment dosyalarına eklenmez. Böylece kişisel çalışma geçmişi workspace paylaşımı sırasında başka kullanıcılara aktarılmaz.

### 9. Cookie yönetimi

Masaüstü sürümü her workspace için kalıcı bir Cookie Jar kullanır. Response ile gelen cookie'ler uygun domain, path, expiry ve güvenlik kurallarına göre saklanır ve sonraki request'lere otomatik olarak uygulanır. Cookie Manager üzerinden kayıtlar görüntülenebilir, düzenlenebilir, etkinleştirilebilir veya silinebilir.

Web sürümünde cookie'ler tarayıcının güvenlik modeli tarafından yönetilir. JavaScript'in `Cookie` ve `Set-Cookie` header'larına erişimi sınırlı olduğundan masaüstündeki Cookie Manager davranışı web üzerinde birebir sunulamaz.

### 10. OpenAPI içe ve dışa aktarma

OpenAPI 3 JSON veya YAML belgeleri Dispatch'e aktarılabilir. İçe aktarma öncesinde endpoint'ler, folder yapısı, server URL'leri, security scheme'leri ve olası veri kayıpları için bir önizleme sunulur.

Bir Dispatch collection'ı OpenAPI 3.0.3 belgesi olarak da dışa aktarılabilir. OpenAPI standardının Dispatch scriptleri ve environment yapısı gibi her özelliği temsil etmediği durumlarda kullanıcıya uyarı gösterilir.

### 11. Workspace yedekleme ve paylaşma

Masaüstü sürümünde workspace, `.dispatch` uzantılı bir arşiv olarak dışa aktarılabilir:

- **Backup:** Collection, environment ve cookie değerlerini eksiksiz taşımayı amaçlar.
- **Safe Share:** Environment değerlerini boşaltır ve cookie'leri arşive dahil etmez.

`.dispatch` arşivleri bütünlük, boyut, path güvenliği ve schema bilgileri açısından doğrulanır. İçe aktarılan arşiv mevcut workspace'in üzerine sessizce yazılmaz; yeni ve bağımsız bir workspace olarak oluşturulur.

## Dispatch nasıl çalışır?

Bir request'in temel çalışma akışı aşağıdaki gibidir:

1. Kullanıcı method, URL, header, query, body ve authentication bilgilerini girer.
2. Varsa pre-request script çalıştırılır.
3. Aktif environment içindeki `{{variable}}` ifadeleri çözümlenir.
4. Authentication ve request ayarları gönderilecek request'e uygulanır.
5. Masaüstünde Rust `reqwest`, web'de tarayıcı `fetch` motoru request'i gönderir.
6. Response ortak bir sonuç modeline dönüştürülür.
7. Varsa post-response script ve testler çalıştırılır.
8. Body, header, cookie, test ve console sonuçları arayüzde gösterilir.
9. Request history'ye eklenir; kullanıcı isterse collection içine kaydeder.

## Web ve masaüstü sürümleri

Temel arayüz ve workspace modeli ortak olsa da tarayıcı güvenliği nedeniyle bazı yetenekler farklılık gösterir.

| Özellik | Masaüstü | Web/PWA |
| --- | --- | --- |
| HTTP motoru | Rust `reqwest` | Browser `fetch` |
| CORS | Tarayıcı CORS kısıtlaması yok | Sunucu CORS izni vermelidir |
| Workspace erişimi | Native dosya sistemi | File System Access API |
| Cookie yönetimi | Dispatch Cookie Jar | Tarayıcı tarafından yönetilir |
| HTTP sürümü seçimi | Desteklenir | Tarayıcı yönetir |
| SSL doğrulamasını kapatma | Desteklenir | Tarayıcı yönetir |
| OAuth Authorization Code + PKCE | Desteklenir | Desteklenmez |
| `.dispatch` import/export | Desteklenir | Desteklenmez |
| OpenAPI import/export | Desteklenir | Desteklenir |

Web sürümünde güncel ve File System Access API desteği bulunan Chromium tabanlı bir tarayıcı kullanılması önerilir.

## Veri saklama ve gizlilik yaklaşımı

Bir Dispatch workspace'i genel olarak aşağıdaki yapıyı kullanır:

```text
Workspace/
├── dispatch.workspace.json
├── collections.json
├── environments.json
├── cookies.json
└── assets/
```

- `dispatch.workspace.json`, workspace kimliği ve format bilgisini taşır.
- `collections.json`, collection, folder, request ve scriptleri saklar.
- `environments.json`, environment ve değişkenleri saklar.
- `cookies.json`, masaüstü Cookie Jar verisini saklar.
- `assets/`, workspace ile ilişkili dosyalar için ayrılmıştır.

History, aktif environment seçimi, tema ve cihaz tercihleri workspace dosyalarından ayrı tutulur. Masaüstünde uygulamanın app-data dizini, web'de ise IndexedDB ve LocalStorage kullanılır.

Dispatch'in local-first olması nedeniyle aşağıdaki güvenlik sorumlulukları önemlidir:

- Hassas environment değerleri içeren workspace'ler kontrolsüz paylaşılmamalıdır.
- Paylaşım için mümkünse masaüstündeki Safe Share seçeneği kullanılmalıdır.
- Workspace klasörünün işletim sistemi izinleri ve yedekleme politikası doğru yapılandırılmalıdır.
- Production token ve parolalarının kaynak kod repository'sine eklenmemesine dikkat edilmelidir.

## Örnek kullanım senaryoları

### Yeni geliştirilen endpoint'i doğrulama

Geliştirici endpoint URL'ini ve request body'yi tanımlar, çağrıyı gönderir ve response'u inceler. Ardından status code ve beklenen alanlar için post-response testleri ekleyerek aynı kontrolü tekrar kullanılabilir hâle getirir.

### Ortamlar arasında geçiş

Aynı collection için `Local`, `Test` ve `Production` environment'ları oluşturulur. URL ve kimlik bilgileri request'e doğrudan yazılmak yerine değişkenlerle tanımlanır. Kullanıcı aktif environment'ı değiştirerek request'i yeniden düzenlemeden farklı hedeflere çağrı yapabilir.

### Uçtan uca API akışını çalıştırma

Kullanıcı oluşturma, oturum açma, kaynak oluşturma ve kaynağı silme request'leri aynı collection içinde sıralanır. Scriptler bir response'tan alınan kimliği environment'a yazar ve sonraki request bu değeri kullanır. Collection Runner bütün akışı sıralı biçimde çalıştırır.

### OpenAPI belgesinden çalışma alanı oluşturma

Bir API'nin OpenAPI belgesi içe aktarılır. Dispatch endpoint'leri collection ve folder'lara dönüştürür; server URL'i için environment oluşturabilir. Kullanıcı oluşan request'leri düzenleyip doğrudan test etmeye başlayabilir.

### Workspace paylaşma

Bir ekip üyesi collection ve environment anahtarlarını içeren workspace'i Safe Share arşivi olarak dışa aktarır. Hassas environment değerleri ve cookie'ler paylaşım dışında bırakılır. Diğer kullanıcı arşivi yeni bir workspace olarak içe aktarır ve kendi değerlerini tanımlar.

## Mevcut sınırlar

Dispatch kullanılırken aşağıdaki mevcut sınırlamalar dikkate alınmalıdır:

- Proje aktif geliştirme aşamasındadır; veri modeli ve fonksiyonlar değişebilir.
- Web sürümü CORS, cookie ve dosya sistemi açısından tarayıcı kısıtlamalarına tabidir.
- Web sürümünde OAuth Authorization Code için yerel callback listener bulunmaz.
- Async scriptler ve async `pm.test` henüz desteklenmez.
- Postman Sandbox API'sinin tamamı değil, belgelenmiş bir alt kümesi desteklenir.
- Collection Runner sonuçları kalıcı geçmiş olarak saklanmaz.
- Devam eden bir HTTP request `Stop run` ile zorla kesilmez; yalnızca yeni request'lerin başlaması durdurulur.
- Aynı workspace klasörünü birden fazla Dispatch instance'ında eşzamanlı düzenlemek için gerçek zamanlı locking veya merge desteği yoktur.
- OpenAPI formatı Dispatch'e özgü script ve environment özelliklerinin tamamını taşıyamaz.
- Performance Runner özel amaçlı yük testi platformlarının bütün dağıtık test ve gözlemleme özelliklerinin yerine geçmez.

## Temel kavramlar

| Kavram | Açıklama |
| --- | --- |
| Workspace | Belirli bir proje veya API kapsamına ait verileri taşıyan ana çalışma alanı |
| Collection | Birbiriyle ilişkili request ve folder'ların grubu |
| Folder | Collection içindeki request'leri hiyerarşik olarak düzenleyen klasör |
| Request | Method, URL, header, query, body, auth ve script bilgilerini taşıyan API çağrısı |
| Environment | Farklı çalışma ortamları için tanımlanan değişken grubu |
| Variable | `{{variable_name}}` biçiminde request alanlarında kullanılan dinamik değer |
| Pre-request script | Request hazırlanmadan önce çalışan JavaScript kodu |
| Post-response script | Response geldikten sonra çalışan test ve işlem kodu |
| Collection Runner | Birden fazla request'i belirli ayarlarla toplu çalıştıran araç |
| Cookie Jar | Masaüstünde cookie'lerin workspace bazında saklandığı yapı |
| OpenAPI | API sözleşmelerini tanımlamak için kullanılan standart belge formatı |
| `.dispatch` | Workspace yedekleme ve paylaşımı için kullanılan Dispatch arşiv formatı |

## İlgili belgeler

- [Ana README](../README.md)
- [Geliştirme ortamının kurulumu](../README.md#gereksinimler)
- [Tek codebase mimarisi](UNIFIED_CODEBASE.md)
- [Workspace mimarisi](WORKSPACE_ARCHITECTURE.md)

