[CmdletBinding()]
param(
    [ValidateSet('release', 'dev')][string]$Profile = 'release',
    [ValidateRange(1, 2)][int]$Jobs = 1
)

$ErrorActionPreference = 'Stop'
$Author = 'carunodev @stfu'
$Root = $PSScriptRoot
$Target = Join-Path $Root 'target-wasm64'
$Names = @('CARGO_BUILD_JOBS', 'CARGO_TARGET_DIR', 'RUSTC_BOOTSTRAP', 'RUSTFLAGS')
$Previous = @{}
foreach ($Name in $Names) { $Previous[$Name] = [Environment]::GetEnvironmentVariable($Name) }
try {
    $env:CARGO_BUILD_JOBS = [string]$Jobs
    $env:CARGO_TARGET_DIR = $Target
    $env:RUSTC_BOOTSTRAP = '1'
    $env:RUSTFLAGS = '-Aexplicit_builtin_cfgs_in_flags --cfg target_feature="atomics" -C target-feature=+atomics,+bulk-memory,+mutable-globals,+simd128 -C link-arg=--shared-memory -C link-arg=--import-memory -C link-arg=--export-table -C link-arg=--export=__stack_pointer -C link-arg=-zstack-size=16777216'
    $Arguments = @('build', '--manifest-path', (Join-Path $Root 'Cargo.toml'), '-p', 'engine-linked', '--lib', '--target', 'wasm64-unknown-unknown', '-Z', 'build-std=std,panic_abort')
    if ($Profile -eq 'release') { $Arguments += '--release' }
    & cargo @Arguments
    if ($LASTEXITCODE -ne 0) { throw "Rust build failed: $LASTEXITCODE" }
    $Folder = if ($Profile -eq 'release') { 'release' } else { 'debug' }
    $Artifact = Join-Path $Target "wasm64-unknown-unknown/$Folder/engine_linked.wasm"
    $Browser = Join-Path $Root 'browser'
    New-Item -ItemType Directory -Path $Browser -Force | Out-Null
    $Temporary = Join-Path $Browser "game.wasm.build-$PID"
    $Destination = Join-Path $Browser 'game.wasm'
    $env:RUSTFLAGS = $null
    $env:CARGO_TARGET_DIR = Join-Path $Root 'target-tools'
    & cargo run --release --quiet --manifest-path (Join-Path $Root '../tools/wasm-package/Cargo.toml') --bin split_memory -- $Artifact $Temporary
    if ($LASTEXITCODE -ne 0) { throw "WASM packaging failed: $LASTEXITCODE" }
    Copy-Item -LiteralPath ([IO.Path]::ChangeExtension($Temporary, 'memory.json')) -Destination (Join-Path $Browser 'memory-config.json')
    & node (Join-Path $Root '../tools/browser_adapter.cjs')
    if ($LASTEXITCODE -ne 0) { throw "Browser adapter failed: $LASTEXITCODE" }
    [IO.File]::Move($Temporary, $Destination, $true)
    Get-FileHash -LiteralPath $Destination
}
finally {
    foreach ($Name in $Names) { [Environment]::SetEnvironmentVariable($Name, $Previous[$Name]) }
}
