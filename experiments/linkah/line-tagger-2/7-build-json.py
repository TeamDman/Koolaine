from pathlib import Path
from tqdm import tqdm
import json
import re

work_dir = Path("work")
data = []
for item in tqdm(list(work_dir.glob("item-*"))):
    rec = item / "rectify"
    if rec.exists():
        generated = rec/"generated.txt"
    else:
        generated = item/"generated.txt"
    response = generated.read_text(encoding="utf-8").split("### Response:",1)[1]
    try:
        response = json.loads(response)
    except json.decoder.JSONDecodeError:
        print(f"\nJSONDecodeError: {generated}", end="")
        continue

    body = item / "request.body.txt"
    assert body.exists()
    body = body.read_text(encoding="utf-8")
    # URL: http://localhost:5000/
    url = re.search(r"URL: (.*)", body).group(1)
    data.append({
        "url": url,
        "tags": response["tags"]
    })
    
outfile = Path("responses.json")
json.dump(data, outfile.open("w", encoding="utf-8"), indent=4)