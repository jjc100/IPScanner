# IPScanner

Windows와 macOS에서 같은 대역의 장비를 빠르게 확인할 수 있는 Rust 기반 GUI IP 스캐너입니다. 한국어, English, 简体中文, Français, Deutsch, हिन्दी UI 전환을 지원하며, 장비 사용 여부, MAC 주소, 제조사, 호스트 이름을 한 번에 확인하고 CSV/XLSX로 저장할 수 있습니다. 프로그램 이름은 **IPScanner**로 표시됩니다.

## 기능

- 단일 IP, 범위(`192.168.1.1-254`), 전체 IP 범위, CIDR(`192.168.1.0/24`) 입력 지원
- 네트워크 어댑터 기준 추천 스캔 범위 자동 표시
- 한국어 / English / 简体中文 / Français / Deutsch / हिन्दी UI 전환
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

## GitHub에 올릴 때 포함되는 구성

- `.gitignore`: 빌드 산출물과 IDE 설정 제외
- `.github/workflows/ci.yml`: GitHub Actions에서 Windows 빌드 확인
- `Cargo.toml`: README 연결 및 패키지 설명 추가
