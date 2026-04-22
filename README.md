# FaIPScanner Clone

윈도우 환경에서 같은 대역의 장비를 빠르게 확인할 수 있는 Rust 기반 GUI IP 스캐너입니다. Korean UI를 기본으로 제공하며, 장비 사용 여부, MAC 주소, 제조사, 호스트 이름을 한 번에 확인하고 CSV/XLSX로 저장할 수 있습니다.

## 기능

- 단일 IP, 범위(`192.168.1.1-254`), 전체 IP 범위, CIDR(`192.168.1.0/24`) 입력 지원
- 네트워크 어댑터 기준 추천 스캔 범위 자동 표시
- 사용 중 / 사용 가능 상태 구분
- MAC 주소와 제조사(OUI) 표시
- DNS / NetBIOS 기반 호스트 이름 조회
- 맵 보기 / 표 보기 전환
- CSV, Excel(`.xlsx`) 결과 저장

## 환경

- Windows
- Rust 1.85+ 권장

이 프로젝트는 Windows API(`IcmpSendEcho`, `SendARP`)와 `ping`, `nbtstat` 명령을 사용하므로 사실상 Windows 전용입니다.

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
