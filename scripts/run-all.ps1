$instances = @('ft70', 'ftv170', 'ftv33', 'kro124p', 'p43', 'rbg323', 'rbg443', 'ry48p')
$times = @(17424127, 727314357, 1024011, 96522704, 2613105, 8227424346, 24451914482, 3221820)
$algorithms = @("random", "random-walk", "nn-heuristic", "greedy-search", "steepest-search")
$cwd = Get-Location
$outputFolder = "./data/results"
$instancesFolder = "./data/ALL_atsp"
$cmdBaseArgs = "run --manifest-path ./atsp_solver/Cargo.toml --release -- "

$runConfigs = @()

foreach ($rep in 1..300) {
    foreach ($algorithm in $algorithms) {
        foreach ($idx in 0..($instances.Length - 1)) {
            $runConfigs += [PSCustomObject]@{
                Rep       = $rep
                Algorithm = $algorithm
                Instance  = $instances[$idx]
                Time      = $times[$idx]
            }
        }
    }
}
Write-Host "Removing old partial results"
Get-ChildItem -Path $outputFolder -Filter "PARTIAL_*.json" | Remove-Item

Write-Host "Running $(($runConfigs | Measure-Object).Count) experiments"
Start-Process -NoNewWindow -Wait -FilePath "cargo" -ArgumentList "build --manifest-path ./atsp_solver/Cargo.toml --release"

$runConfigs | ForEach-Object -Parallel {
    $fileName = "PARTIAL_$($_.Instance)_$($_.Algorithm)_$($_.Rep).json"
    $output = Join-Path $using:outputFolder $fileName
    $instanceFile = Join-Path $using:instancesFolder "$($_.Instance).atsp"
    $cmdArgs = "-i $instanceFile -a $($_.Algorithm) -m $($_.Time) -o $output -t -s $($_.Rep)"
    Write-Host "Running 'cargo $($using:cmdBaseArgs) $cmdArgs'"
    $status = Start-Process "cargo" -ArgumentList "$($using:cmdBaseArgs) $cmdArgs" -Wait -PassThru -WindowStyle Hidden -WorkingDirectory $using:cwd
    if ($status.ExitCode -ne 0) {
        Write-Host "Error running $cmdArgs"
    }
} -ThrottleLimit 16 

Write-Host "All experiments finished"
$allResults = Join-Path $outputFolder "all.json"
Write-Host "Combining results into $allResults"
$combinedData = @()
Get-ChildItem -Path $outputFolder -Filter "PARTIAL_*.json" | ForEach-Object {
    $json = Get-Content $_.FullName | ConvertFrom-Json
    $combinedData += $json 
    Remove-Item $_.FullName
}
$combinedData | ConvertTo-Json | Set-Content $allResults
