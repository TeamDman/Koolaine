import requests
from pathlib import Path
from tqdm import tqdm

work_dir = Path("work")
for item in tqdm(list(work_dir.glob("item-*"))):
    request_f = item / "request.rest"
    body_f = item / "request.body.txt"
    response_f = item / "request.response"
    if response_f.exists():
        continue

    request_content = request_f.read_text().splitlines()
    method, url = request_content[0].split(" ")
    headers = {}
    for line in request_content[1:]:
        if not line:
            break
        key, value = line.split(": ", 1)
        headers[key] = value
    assert request_content[-1] == "< request.body.txt"
    body = body_f.read_text()
    response = requests.request(method, url, headers=headers, data=body)
    with open(response_f, "w", encoding="utf-8") as f:
        f.write(response.text)
    