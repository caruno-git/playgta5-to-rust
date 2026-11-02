[CmdletBinding()]
param(
    [Parameter(Mandatory)][string]$Wasm,
    [string]$JavaScript,
    [string]$ReferenceImage
)

$ErrorActionPreference = 'Stop'
$Author = 'carunodev @stfu'
$Root = $PSScriptRoot
$Original = (Resolve-Path -LiteralPath $Wasm).Path
$Expected = '11ca8d2c04c5e843d18ff4aea4899d72c86973c6b031df334e67c446b2ae83e0'
if ((Get-FileHash -LiteralPath $Original -Algorithm SHA256).Hash -ine $Expected) { throw 'Original game.wasm hash mismatch' }
$Inputs = Join-Path $Root 'inputs'
New-Item -ItemType Directory -Path $Inputs -Force | Out-Null
$Target = Join-Path $Inputs 'game.wasm'
if ($Original -ine $Target) { Copy-Item -LiteralPath $Original -Destination $Target }
& python (Join-Path $Root 'tools/w2r/gen_segments.py') $Target (Join-Path $Root 'wasm-rt/src')
if ($LASTEXITCODE -ne 0) { throw 'Segment extraction failed' }
if ($JavaScript) {
    $From = (Resolve-Path -LiteralPath $JavaScript).Path
    $To = Join-Path $Inputs 'game.js'
    if ($From -ine $To) { Copy-Item -LiteralPath $From -Destination $To }
}
if ($ReferenceImage) {
    $From = (Resolve-Path -LiteralPath $ReferenceImage).Path
    if ((Get-Item -LiteralPath $From).Length -ne 7984420) { throw 'Unexpected reference-image size' }
    $To = Join-Path $Inputs 'va.bin'
    if ($From -ine $To) { Copy-Item -LiteralPath $From -Destination $To }
}
Write-Output 'External inputs prepared locally; inputs and segment blobs are ignored by Git.'
