param (
    [ValidateSet("arm64","linux","x64")]
    [string]$Target = "autodetect",
    [switch]$Release
)

if ($Target -eq "autodetect") {
    if (Test-Path "/etc/os-release") {
        $targetName = "x86_64-unknown-linux-gnu"
        $Target = "linux"
    } elseif ((Get-WmiObject -Class Win32_ComputerSystem).SystemType -match 'x64') {
        $targetName = "x86_64-pc-windows-msvc"
        $Target = "x64"
    } elseif ((Get-WmiObject -Class Win32_ComputerSystem).SystemType -match 'arm64') {
        $targetName = "aarch64-pc-windows-msvc"
        $Target = "arm64"  
    } else {
        Write-Host "Cannot detect system architecture"
        exit 1
    }
} else {
    switch ($Target) {
        "arm64" {
            $targetName = "aarch64-pc-windows-msvc"
        }
        "linux" {
            $targetName = "x86_64-unknown-linux-gnu"
        }
        "x64" {
            $targetName = "x86_64-pc-windows-msvc"
        }
        default {
            Write-Host "Unsupported architecture: $Target"
            exit 1
        }
    }
}

# $env:CARGO_TARGET_DIR = "$env:SystemDrive\Temp\CARGO_TARGET_DIR"

if ($Release) {
    Write-Host -ForegroundColor Yellow "Target OS : $Target"
    Write-Host -ForegroundColor Yellow "Build type: Release"
    # Write-Host -ForegroundColor Yellow "Target folder: $env:CARGO_TARGET_DIR"
    cargo build --target $targetName --release
} else {
    Write-Host -ForegroundColor Yellow "Target OS : $Target"
    Write-Host -ForegroundColor Yellow "Build type: Debug"
    # Write-Host -ForegroundColor Yellow "Target folder: $env:CARGO_TARGET_DIR"
    cargo build --target $targetName
}
