# 1. 커널 빌드
cargo build
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

# 2. iso_root 디렉터리 세팅
New-Item -ItemType Directory -Force -Path "iso_root/EFI/BOOT" | Out-Null
Copy-Item "target/x86_64-unknown-none/debug/daisy-kernel" -Destination "iso_root/daisy-kernel" -Force
Copy-Item "limine.conf" -Destination "iso_root/limine.conf" -Force

# 3. QEMU 실행 (OVMF UEFI 펌웨어 로드 + 시리얼 콘솔 표준출력 연결)
# OVMF.fd 경로는 설치된 위치에 맞게 지정 (예: C:\Program Files\qemu\share\edk2-x86_64-code.fd 등)
qemu-system-x86_64 `
    -bios "C:\Program Files\qemu\share\edk2-x86_64-code.fd" `
    -drive format=raw,file=fat:rw:iso_root `
    -serial stdio `
    -m 512M `
    -no-reboot