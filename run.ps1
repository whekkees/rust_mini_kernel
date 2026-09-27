$ErrorActionPreference = "Stop"

$kernel = $args[0]

Write-Host "==> Kernel: $kernel"

Write-Host "==> Building ISO..."
Copy-Item `
    $kernel `
    "iso\boot\os" `
    -Force

Write-Host "==> Building ISO with GRUB..."
wsl bash -c "cd /mnt/c/sancikovich/averion_os/os && grub-mkrescue -o os.iso iso"

Write-Host "==> Starting QEMU..."
qemu-system-i386 -cdrom os.iso -m 512M -vga std -audiodev dsound,id=speaker -machine pcspk-audiodev=speaker