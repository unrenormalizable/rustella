param(
  [Parameter(Mandatory)]
  [String] $AsmFile

  ,[String] $OutName
)

$toolsDir = $PSScriptRoot

$asmFile = $AsmFile | Resolve-Path
$asmDir = Split-Path $asmFile
$asmFileName = $OutName ? $OutName : (Split-Path $asmFile -LeafBase )

$flags = @("-DNO_ILLEGAL_OPCODES=1")

$incDirs = @("$toolsDir\machines\atari2600", $asmDir, (Join-Path $asmDir "grx"), (Join-Path $asmDir "playfields"), (Join-Path $asmDir "purrballs")) | % { "-I$_" }

Write-Host -ForegroundColor Green "... DASM: $asmFile"

del "$asmDir\$asmFileName.bin"
del "$asmDir\$asmFileName.sym"
del "$asmDir\$asmFileName.lst"
& "$toolsDir\dasm.exe" "$asmFile" -f3 -v0 @incDirs "-o$asmDir\$asmFileName.bin" "-s$asmDir\$asmFileName.sym" "-l$asmDir\$asmFileName.lst" @flags
