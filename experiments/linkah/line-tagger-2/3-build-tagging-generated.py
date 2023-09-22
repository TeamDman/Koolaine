from pathlib import Path
from tqdm import tqdm
import json

work_dir = Path("work")
for item in tqdm(list(work_dir.glob("item-*"))):
    prompt = (item / "request.body.txt").read_text(encoding="utf-8")
    response_f = item / "request.response"
    response_txt = response_f.read_text(encoding="utf-8")
    try:
        response_json = json.loads(response_txt)
    except json.decoder.JSONDecodeError:
        print("JSONDecodeError on", item)
        continue
    outfile = item / "generated.txt"
    with outfile.open('w', encoding="utf-8") as f:
        f.write(prompt + response_json["results"][0]["text"])