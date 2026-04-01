# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

이 저장소는 Claude Code 활용법을 학습하기 위한 스터디 프로젝트입니다.
Claude Code의 기능(CLAUDE.md, Memory, Skills, Hooks 등)을 실습하고, 서브 프로젝트를 통해 실제 개발도 함께 진행합니다.

## Sub Project: MediaMTX Manager

MediaMTX를 활용한 RTSP 서버 관리 애플리케이션.

### 실행 모드
- **데스크탑 모드**: Tauri 데스크탑 앱 (일반 사용자 환경)
- **웹 모드**: 동일한 React UI를 웹 서버로 제공 (서버/headless 환경에서 브라우저 접속)
- 동일한 React 프론트엔드를 데스크탑과 웹 모드에서 공유

### 기술 스택
- **데스크탑**: Tauri 2.x (Rust 백엔드 + React/TypeScript 프론트엔드)
- **웹 서버**: Rust (axum 등) — headless 환경에서 React UI를 서빙 + REST API 제공
- **대상 플랫폼**: macOS, Windows, Linux

### MediaMTX란?
- Go 기반 오픈소스 미디어 서버 ([bluenviron/mediamtx](https://github.com/bluenviron/mediamtx))
- RTSP/RTMP/HLS/WebRTC 지원
- REST API 제공 (기본 포트 9997)
- YAML 기반 설정 파일

### 기능 목록

#### Phase 1: 코어
- MediaMTX 바이너리 다운로드/업데이트 및 프로세스 시작/중지
- 경로(Path) 관리 — 다중 스트림 경로 CRUD
- 설정 파일(YAML) 조회/편집 + Hot Reload (클라이언트 연결 유지한 채 설정 반영)

#### Phase 2: 스트림 관리
- **Publish**: SRT, WebRTC, RTSP, RTMP, HLS, MPEG-TS, RTP 소스 발행 설정
- **Read/Play**: SRT, WebRTC, RTSP, RTMP, HLS 스트림 구독/재생
- 프로토콜 간 자동 변환 설정
- Always-available 스트림 설정 (발행자 오프라인 시에도 경로 유지)

#### Phase 3: 녹화 & 재생
- 스트림 녹화 설정 (fMP4 / MPEG-TS 포맷)
- 녹화된 스트림 목록 조회 및 재생

#### Phase 4: 인증 & 라우팅
- 인증 설정 (Internal / HTTP / JWT)
- 스트림 포워딩 (다른 서버로 전달)
- 프록시 설정 (다른 서버 요청 중계)

#### Phase 5: 모니터링 & 운영
- 연결된 클라이언트 및 스트림 상태 실시간 모니터링
- Prometheus 메트릭 대시보드
- CPU/RAM 성능 모니터링
- Hooks 관리 (클라이언트 connect/disconnect/read/publish 이벤트에 외부 명령 실행)

## Git 관리 원칙

### 커밋 메시지
- Conventional Commits 형식 사용: `type: 설명`
- type: `init`, `feat`, `fix`, `refactor`, `docs`, `test`, `chore`, `ci`
- 설명은 한국어로 작성
- 예: `feat: 경로 관리 CRUD API 추가`, `fix: 프로세스 종료 시 좀비 프로세스 방지`

### 브랜치 전략
- `main` — 안정된 코드, 직접 커밋 가능 (스터디 프로젝트이므로)
- `feat/<기능명>` — 새로운 기능 개발 시 분기
- `fix/<이슈>` — 버그 수정 시 분기
- 기능 완료 후 main에 머지

### 커밋 단위
- 하나의 커밋은 하나의 논리적 변경 단위
- 여러 기능을 한 커밋에 섞지 않는다
- 빌드가 깨지지 않는 상태로 커밋한다

## 코드 리뷰

### 워크플로우
1. 기능 구현
2. 코드 리뷰 요청 → 리뷰 결과 확인 → 수정
3. 커밋

### 리뷰 범위
- **코드 품질**: 가독성, 중복, 네이밍, 에러 처리
- **아키텍처**: 의존성 방향, 레이어 분리, core/tauri/web 경계 준수
- **보안**: 민감정보 노출, 입력 검증, 의존성 취약점
- **TypeScript**: 타입 안정성, any 사용 지양, 타입가드 패턴

### 리뷰 시점
- 새로운 기능 구현 완료 시
- 복잡한 리팩토링 후
- PR 생성 전

## Language

사용자는 한국어를 사용합니다. 모든 응답은 한국어로 작성합니다.
