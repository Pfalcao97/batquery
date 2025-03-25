$hearbeat=10

Do {
    $ProgramRunning = Get-Process | Select-String -Pattern geekbench
    $BenchmarkRunning = Get-Process | Select-String -Pattern geekbench_avx2

    if ($ProgramRunning){
    
        Write-Host "Geekbench 6 is open."

        If ($BenchmarkRunning) {
            Write-Host "Performing a benchmark."
            $benchmark="true"
        }else{
            $benchmark="false"
        }

        cargo run -- $benchmark
        Start-Sleep $hearbeat   
    }

} Until (!$ProgramRunning)

# Powershell.exe -File .\benchmark.ps1