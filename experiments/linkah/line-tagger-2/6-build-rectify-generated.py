from pathlib import Path
from tqdm import tqdm
import json

work_dir = Path("work")
for item in tqdm(list(work_dir.glob("item-*"))):
    rec = item / "rectify"
    if not rec.exists():
        continue
    prompt = (rec / "request.body.txt").read_text(encoding="utf-8")
    response_f = rec / "request.response"
    response_txt = response_f.read_text(encoding="utf-8")
    try:
        response_json = json.loads(response_txt)
    except json.decoder.JSONDecodeError:
        print("JSONDecodeError on", rec)
        continue
    outfile = rec / "generated.txt"
    with outfile.open('w', encoding="utf-8") as f:
        f.write(prompt + response_json["results"][0]["text"])
    print("\n", outfile, end="")