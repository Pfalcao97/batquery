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

        batbench benchmark -f "C:\Users\Pedro Falcao\Documents\Rust\battery-script\windows\results\base\benchmark.csv" --benchmark $benchmark
        Start-Sleep $hearbeat   
    }

} Until (!$ProgramRunning)