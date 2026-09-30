$ErrorActionPreference = "Stop"

$Root = $PSScriptRoot
Set-Location $Root

$Kernel = "$Root\target\i686-averion\debug\os"

if (!(Test-Path $Kernel)) {
    $Kernel = "$Root\target\i686-averion\debug\os.exe"
}

if (!(Test-Path $Kernel)) {
    throw "Kernel not found."
}

$WslKernel = "/mnt/c/sancikovich/averion_os/os/target/i686-averion/debug/os"

wsl grub-file --is-x86-multiboot "$WslKernel"

if ($LASTEXITCODE -ne 0) {
    $WslKernel = "/mnt/c/sancikovich/averion_os/os/target/i686-averion/debug/os.exe"
    wsl grub-file --is-x86-multiboot "$WslKernel"
}

if ($LASTEXITCODE -ne 0) {
    throw "Multiboot header not found."
}

New-Item -ItemType Directory -Force -Path "$Root\iso\boot" | Out-Null

Copy-Item $Kernel "$Root\iso\boot\os" -Force

wsl bash -c "cd /mnt/c/sancikovich/averion_os/os && grub-mkrescue -o os.iso iso"

if ($LASTEXITCODE -ne 0) {
    throw "grub-mkrescue failed."
}

qemu-system-i386 -cdrom "$Root\os.iso" -m 512M -vga std -audiodev dsound,id=speaker -machine pcspk-audiodev=speaker