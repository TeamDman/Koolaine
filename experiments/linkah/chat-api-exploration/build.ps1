$content = @{
    prompt=$(Get-Content -Raw -Path prompt.txt);
    max_new_tokens=500;
}
$content | ConvertTo-Json > prompt.json

python .\restanal.py .\chat.rest > chat.rest.json