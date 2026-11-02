# 🌼 Project Daisy - Development Roadmap & TODO

Project Daisy는 x86_64 아키텍처 기반의 Rust 마이크로커널 프로젝트입니다.

---

## 🚀 Milestone 1: 부팅 및 기본 아키텍처 (완료)
- [x] Limine 부트로더 연동 및 Higher-Half 커널 로드
- [x] 16550 UART 기반 시리얼 포트 드라이버 구현
- [x] 64비트 GDT(Global Descriptor Table) 구성 및 세그먼트 레지스터 재장전
- [x] 64비트 IDT(Interrupt Descriptor Table) 및 기본 CPU 예외 핸들러 등록
- [x] `int3` 소프트웨어 인터럽트(Breakpoint) 예외 처리 검증

---

## 📦 Milestone 2: 메모리 관리 서브시스템 (진행 예정)

### 🧩 물리 메모리 관리자 (PMM - Physical Memory Manager)
- [ ] Limine `MemoryMapRequest`를 통한 UEFI 메모리 맵 파싱
- [ ] 비트맵(Bitmap) 기반 물리 프레임 할당자 구현 (`alloc_frame`, `free_frame`)
- [ ] 부팅 시 초기 비트맵 배치를 위한 가용 메모리 영역 탐색

### 🗺️ 가상 메모리 관리자 (VMM - Virtual Memory Manager)
- [ ] 4단계 페이징(PML4 -> PDPT -> PD -> PT) 구조체 정의
- [ ] HHDM(Higher-Half Direct Map)을 활용한 페이지 테이블 매핑 함수 구현
- [ ] 유저 공간 / 커널 공간 메모리 보호 플래그 설정 (NX, User/Supervisor)

### 힙 할당자 (Heap Allocator)
- [ ] 커널용 동적 메모리 할당(`alloc`, `Box`, `Vec` 등) 지원
- [ ] Linked List 또는 Buddy Allocator 기반 글로벌 할당자 등록

---

## 👨‍👦 협업 역할 분담 (Role Delegation)

### 🧑‍💻 Blshu (Core Architecture & Driver)
- [ ] PMM 비트맵 할당자 핵심 알고리즘 작성
- [ ] 커널 패닉 핸들러 고도화 (레지스터 덤프 및 스택 트레이스)
- [ ] APIC(Advanced Programmable Interrupt Controller) 타이머 드라이버 세팅

### 👨‍💻 Pluto-sir (Infrastructure, Memory & Verification)
- [ ] VMM 4단계 페이징 매핑 및 TLB 플러시 유틸리티 함수 구현
- [ ] 커널 힙 할당자(`GlobalAlloc`) 인터페이스 연동 및 안정성 테스트
- [ ] CI 워크플로우 보강 (QEMU 무인 통합 테스트 스크립트 작성)

---

## 🔮 Milestone 3: 태스크 및 마이크로커널 IPC (장기 목표)
- [ ] TSS(Task State Segment) 및 스택 스위칭 구현
- [ ] 유저 모드(Ring 3) 진입 및 Syscall 인터페이스 구성
- [ ] 동기식/비동기식 메시지 패싱 기반 마이크로커널 IPC 프로토콜 설계