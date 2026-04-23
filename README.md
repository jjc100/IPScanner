# IPScanner

Windows와 macOS에서 같은 대역의 장비를 빠르게 확인할 수 있는 Rust 기반 GUI IP 스캐너입니다. 대한민국·한국어, United States·English, 中国·简体中文, France·Français, Deutschland·Deutsch, भारत·हिन्दी 로케일 전환을 지원하며, 장비 사용 여부, MAC 주소, 제조사, 호스트 이름을 한 번에 확인하고 CSV/XLSX로 저장할 수 있습니다. 프로그램 이름은 **IPScanner**로 표시됩니다.

## Localized guides

### 대한민국 · 한국어

IPScanner는 같은 네트워크 대역의 장비를 빠르게 찾고, 한 화면에서 상태를 확인할 수 있도록 만든 데스크톱 IP 스캐너입니다. 사무실, 집, 테스트 네트워크처럼 여러 장비가 연결된 환경에서 어떤 주소가 사용 중인지, 어떤 장비가 응답하는지 쉽게 파악할 수 있습니다.

**주요 기능**
- 단일 IP, 범위(`192.168.1.1-254`), 전체 구간, CIDR(`192.168.1.0/24`) 스캔
- 네트워크 어댑터를 기준으로 추천 스캔 범위 자동 제안
- 사용 중 장비 / 사용 가능 주소 구분
- MAC 주소, 제조사(OUI), 호스트 이름 확인
- 맵 보기와 표 보기 전환
- CSV 및 XLSX 내보내기
- 국가/언어 전환 지원

**지원 환경**
- Windows
- macOS
- Rust 1.85+ 권장

Windows에서는 네이티브 ICMP/ARP API와 시스템 유틸리티를 활용하고, macOS에서는 시스템 `ping`/`arp`를 사용합니다. 플랫폼에 따라 호스트 이름 수집 범위는 일부 다를 수 있습니다.

**실행**
```powershell
cargo run
```

**테스트**
```powershell
cargo test
```

**릴리스 다운로드**
- `v0.1.0` 같은 태그를 푸시하면 GitHub Actions가 Windows/macOS 빌드를 생성합니다.
- Release를 publish 하면 실행 파일 자산이 함께 업로드됩니다.
- 사용자는 GitHub Releases 페이지에서 바로 다운로드해 사용할 수 있습니다.

### United States · English

IPScanner is a desktop IP scanner designed to help users quickly inspect devices on the same network segment and understand address usage at a glance. It is useful for home networks, office LANs, labs, and other environments where you need to see which hosts respond and which IPs are already in use.

**Key features**
- Scan a single IP, a range such as `192.168.1.1-254`, a full span, or CIDR blocks like `192.168.1.0/24`
- Suggest scan ranges automatically from the selected network adapter
- Distinguish active devices from available addresses
- Show MAC addresses, OUI vendor names, and hostnames
- Switch between map view and table view
- Export results to CSV or XLSX
- Change the UI by country/language locale

**Supported environment**
- Windows
- macOS
- Rust 1.85+ recommended

On Windows, IPScanner uses native ICMP/ARP APIs together with trusted system utilities. On macOS, it uses the system `ping` and `arp` tools. Hostname resolution can vary slightly depending on platform capabilities.

**Run**
```powershell
cargo run
```

**Test**
```powershell
cargo test
```

**Release downloads**
- Pushing a tag such as `v0.1.0` triggers GitHub Actions to build Windows and macOS artifacts.
- Publishing a GitHub Release also runs the upload workflow.
- End users can download ready-to-use builds directly from the Releases page.

### 中国 · 简体中文

IPScanner 是一款桌面 IP 扫描工具，用于快速查看同一网段中的设备，并帮助用户直观了解哪些地址正在使用。它适合家庭网络、办公室局域网、实验环境等场景，方便确认哪些主机在线、哪些地址已经被占用。

**主要功能**
- 支持扫描单个 IP、范围（如 `192.168.1.1-254`）、完整地址段以及 CIDR（如 `192.168.1.0/24`）
- 根据所选网卡自动推荐扫描范围
- 区分在线设备与可用地址
- 显示 MAC 地址、厂商（OUI）和主机名
- 支持在地图视图与表格视图之间切换
- 支持导出为 CSV 或 XLSX
- 支持按国家/语言切换界面

**支持环境**
- Windows
- macOS
- 建议使用 Rust 1.85+

在 Windows 上，IPScanner 会使用原生 ICMP/ARP API 以及可信系统工具；在 macOS 上，则使用系统自带的 `ping` 和 `arp`。由于平台能力不同，主机名解析结果可能会有少量差异。

**运行**
```powershell
cargo run
```

**测试**
```powershell
cargo test
```

**发布与下载**
- 推送 `v0.1.0` 这类标签后，GitHub Actions 会自动构建 Windows 和 macOS 版本。
- 发布 GitHub Release 时，也会自动上传可下载文件。
- 用户可以直接从 Releases 页面下载并使用。

### France · Français

IPScanner est un scanner IP de bureau conçu pour repérer rapidement les appareils présents sur le meme segment reseau et comprendre l'occupation des adresses d'un simple coup d'oeil. Il convient aux reseaux domestiques, aux bureaux, aux laboratoires et a tout environnement ou il faut identifier rapidement les hotes actifs.

**Fonctionnalites principales**
- Analyse d'une IP unique, d'une plage comme `192.168.1.1-254`, d'une plage complete ou d'un bloc CIDR tel que `192.168.1.0/24`
- Proposition automatique de plages de scan a partir de l'adaptateur reseau selectionne
- Distinction entre appareils actifs et adresses disponibles
- Affichage des adresses MAC, du fabricant (OUI) et du nom d'hote
- Bascule entre vue carte et vue tableau
- Export des resultats en CSV ou XLSX
- Changement de langue/interface par pays et langue

**Environnement pris en charge**
- Windows
- macOS
- Rust 1.85+ recommande

Sous Windows, IPScanner utilise les API natives ICMP/ARP et des utilitaires systeme fiables. Sous macOS, il s'appuie sur `ping` et `arp` fournis par le systeme. La resolution des noms d'hote peut varier legerement selon la plateforme.

**Execution**
```powershell
cargo run
```

**Tests**
```powershell
cargo test
```

**Publication et telechargement**
- Un tag comme `v0.1.0` declenche GitHub Actions pour creer les builds Windows et macOS.
- La publication d'une GitHub Release lance egalement le workflow de mise en ligne des fichiers.
- Les utilisateurs peuvent telecharger directement les binaires depuis la page Releases.

### Deutschland · Deutsch

IPScanner ist ein Desktop-IP-Scanner, mit dem sich Geraete im selben Netzwerksegment schnell finden und belegte Adressen auf einen Blick erkennen lassen. Das Programm eignet sich fuer Heimnetzwerke, Buero-LANs, Labore und andere Umgebungen, in denen aktive Hosts und freie IP-Adressen schnell ueberprueft werden muessen.

**Wichtige Funktionen**
- Scan einer einzelnen IP, eines Bereichs wie `192.168.1.1-254`, eines kompletten Bereichs oder von CIDR-Netzen wie `192.168.1.0/24`
- Automatische Vorschlaege fuer Scan-Bereiche anhand des ausgewaehlten Netzwerkadapters
- Unterscheidung zwischen aktiven Geraeten und verfuegbaren Adressen
- Anzeige von MAC-Adressen, Herstellerinformationen (OUI) und Hostnamen
- Wechsel zwischen Kartenansicht und Tabellenansicht
- Export der Ergebnisse als CSV oder XLSX
- Umschalten der Benutzeroberflaeche nach Land/Sprache

**Unterstuetzte Umgebung**
- Windows
- macOS
- Rust 1.85+ empfohlen

Unter Windows verwendet IPScanner native ICMP/ARP-APIs und vertrauenswuerdige Systemprogramme. Unter macOS werden die eingebauten Werkzeuge `ping` und `arp` genutzt. Die Hostnamen-Ermittlung kann sich je nach Plattform leicht unterscheiden.

**Ausfuehren**
```powershell
cargo run
```

**Tests**
```powershell
cargo test
```

**Release und Download**
- Wenn ein Tag wie `v0.1.0` gepusht wird, erstellt GitHub Actions Windows- und macOS-Artefakte.
- Beim Veroeffentlichen einer GitHub Release werden die Dateien ebenfalls automatisch hochgeladen.
- Benutzer koennen die fertigen Pakete direkt von der Releases-Seite herunterladen.

### भारत · हिन्दी

IPScanner एक डेस्कटॉप IP स्कैनर है, जिसे एक ही नेटवर्क सेगमेंट में मौजूद डिवाइस को जल्दी पहचानने और उपयोग में आ रहे पते एक नज़र में समझने के लिए बनाया गया है। यह घर, कार्यालय, लैब या किसी भी ऐसे नेटवर्क वातावरण में उपयोगी है जहाँ सक्रिय होस्ट और उपलब्ध IP पते जल्दी देखना जरूरी हो।

**मुख्य सुविधाएँ**
- एकल IP, रेंज जैसे `192.168.1.1-254`, पूरा पता दायरा, और CIDR जैसे `192.168.1.0/24` स्कैन करना
- चुने गए नेटवर्क एडेप्टर के आधार पर अनुशंसित स्कैन रेंज दिखाना
- सक्रिय डिवाइस और उपलब्ध पते अलग-अलग दिखाना
- MAC पता, निर्माता (OUI) और होस्टनाम दिखाना
- मैप व्यू और टेबल व्यू के बीच बदलना
- परिणामों को CSV या XLSX में निर्यात करना
- देश/भाषा के अनुसार UI बदलना

**समर्थित वातावरण**
- Windows
- macOS
- Rust 1.85+ अनुशंसित

Windows पर IPScanner नेटिव ICMP/ARP API और विश्वसनीय सिस्टम यूटिलिटी का उपयोग करता है। macOS पर यह सिस्टम के `ping` और `arp` टूल का उपयोग करता है। प्लेटफॉर्म के अनुसार होस्टनाम प्राप्त करने का परिणाम थोड़ा अलग हो सकता है।

**चलाना**
```powershell
cargo run
```

**टेस्ट**
```powershell
cargo test
```

**रिलीज़ और डाउनलोड**
- `v0.1.0` जैसे टैग को push करने पर GitHub Actions Windows और macOS बिल्ड तैयार करता है।
- GitHub Release publish करने पर डाउनलोड योग्य फ़ाइलें भी अपलोड की जाती हैं।
- उपयोगकर्ता Releases पेज से तैयार बिल्ड सीधे डाउनलोड करके चला सकते हैं।

## 기능

- 단일 IP, 범위(`192.168.1.1-254`), 전체 IP 범위, CIDR(`192.168.1.0/24`) 입력 지원
- 네트워크 어댑터 기준 추천 스캔 범위 자동 표시
- 국가별 로케일 전환 지원: 대한민국 / United States / 中国 / France / Deutschland / भारत
- 사용 중 / 사용 가능 상태 구분
- MAC 주소와 제조사(OUI) 표시
- DNS / NetBIOS 기반 호스트 이름 조회
- 맵 보기 / 표 보기 전환, 요약 카드, 더 친숙한 대시보드 UI
- CSV, Excel(`.xlsx`) 결과 저장

## 환경

- Windows
- macOS
- Rust 1.85+ 권장

Windows에서는 네이티브 ICMP/ARP API와 시스템 유틸리티를 사용하고, macOS에서는 시스템 `ping`/`arp` 유틸리티를 사용합니다. 호스트 이름 조회는 플랫폼에 따라 가능한 정보 범위가 조금 다를 수 있습니다.

## 실행

```powershell
cargo run
```

## 테스트

```powershell
cargo test
```

## 배포

- `v0.1.0` 같은 **태그를 push**하면 GitHub Actions가 Windows/macOS용 실행 파일을 빌드해서 Release 자산으로 업로드합니다.
- GitHub에서 **Release를 publish**해도 같은 워크플로가 자산 업로드를 처리합니다.
- 사용자는 Releases 페이지에서 바로 내려받아 실행할 수 있습니다.

## GitHub에 올릴 때 포함되는 구성

- `.gitignore`: 빌드 산출물과 IDE 설정 제외
- `.github/workflows/ci.yml`: GitHub Actions에서 Windows 빌드 확인
- `.github/workflows/release.yml`: Windows/macOS 릴리스 자산 자동 생성 및 업로드
- `Cargo.toml`: README 연결 및 패키지 설명 추가

## Repository details by locale

### United States · English
- `.gitignore`: excludes build artifacts and IDE settings
- `.github/workflows/ci.yml`: validates the Windows build in GitHub Actions
- `.github/workflows/release.yml`: creates and uploads Windows/macOS release assets automatically
- `Cargo.toml`: connects the README and package metadata

### 中国 · 简体中文
- `.gitignore`：排除构建产物和 IDE 配置
- `.github/workflows/ci.yml`：在 GitHub Actions 中检查 Windows 构建
- `.github/workflows/release.yml`：自动生成并上传 Windows/macOS 发布文件
- `Cargo.toml`：关联 README 并包含包元数据

### France · Français
- `.gitignore` : exclut les artefacts de build et les reglages d'IDE
- `.github/workflows/ci.yml` : verifie le build Windows dans GitHub Actions
- `.github/workflows/release.yml` : cree et charge automatiquement les artefacts de release Windows/macOS
- `Cargo.toml` : relie le README et les metadonnees du paquet

### Deutschland · Deutsch
- `.gitignore`: schliesst Build-Artefakte und IDE-Einstellungen aus
- `.github/workflows/ci.yml`: prueft den Windows-Build in GitHub Actions
- `.github/workflows/release.yml`: erstellt und laedt Windows/macOS-Release-Artefakte automatisch hoch
- `Cargo.toml`: verknuepft README und Paketmetadaten

### भारत · हिन्दी
- `.gitignore`: बिल्ड फ़ाइलें और IDE सेटिंग्स बाहर रखता है
- `.github/workflows/ci.yml`: GitHub Actions में Windows बिल्ड की जाँच करता है
- `.github/workflows/release.yml`: Windows/macOS रिलीज़ फ़ाइलें अपने आप बनाकर अपलोड करता है
- `Cargo.toml`: README और पैकेज मेटाडेटा को जोड़ता है
