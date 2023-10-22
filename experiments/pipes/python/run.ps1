if ($env:CONDA_DEFAULT_ENV -ne "torch") {
    Write-Host "wrong env active, activating torch"
    conda activate torch
}
python main.py s