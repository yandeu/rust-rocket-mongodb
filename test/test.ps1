$proc = Start-Process ./target/debug/my-app.exe -passthru

Start-Sleep 1

cargo test

$ERRCODE=$LASTEXITCODE

taskkill /pid $proc.Id /f

Write-Output "CODE: $ERRCODE"

Start-Sleep 2

exit $ERRCODE