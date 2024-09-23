param(
  [Parameter(Mandatory)]
  [String] $AsmFile

  ,[String] $OutName
)

$asmFile = $AsmFile | Resolve-Path
$asmDir = Split-Path $asmFile
$toolsDir = Join-Path $PSScriptRoot "../../../tools" | Resolve-Path
$asmFileName = $OutName ? $OutName : (Split-Path $asmFile -LeafBase )

$flags = @("-DNO_ILLEGAL_OPCODES=1")

& "$toolsDir\dasm.exe" "$asmFile" -f3 -v0 "-I$toolsDir\machines\atari2600" "-I$asmDir" "-o$asmDir\$asmFileName.bin" "-s$asmDir\$asmFileName.sym" "-l$asmDir\$asmFileName.lst" @flags
