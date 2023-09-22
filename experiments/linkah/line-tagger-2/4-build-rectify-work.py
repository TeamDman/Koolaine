import os
from pathlib import Path
import jinja2
from pathlib import Path
from tqdm import tqdm
import json

templates = {f.name: f.read_text(encoding="utf-8") for f in Path("templates/rectify").glob("*.jinja")}

work_dir = Path("work")
for item in tqdm(list(work_dir.glob("item-*"))):
    generated = (item/"generated.txt").read_text(encoding="utf-8").splitlines()
    response = "\n".join(generated[6:])
    try:
        json.loads(response)
    except json.decoder.JSONDecodeError:
        newwork = item / "rectify"
        newwork.mkdir(parents=True, exist_ok=True)
        req = newwork / "request.rest"
        body = newwork / "request.body.txt"
        print("\n", req, body, end="")
        with open(req, "w", encoding="utf-8") as f:
            content = jinja2.Template(templates["request.rest.jinja"]).render()
            f.write(content)
        with open(body, "w", encoding="utf-8") as f:
            content = jinja2.Template(templates["request.body.txt.jinja"]).render(
                json=response,
            )
            f.write(content)
