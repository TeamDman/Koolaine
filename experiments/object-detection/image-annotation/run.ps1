$dir = "D:\Repos\Koolaine\experiments\object-detection\screenshot-dataset\filtered-screenshots\"
$file = Get-ChildItem -Path $dir | Get-Random
Write-Host "Running with file: $file"
cargo run -- `
    --image "$file"