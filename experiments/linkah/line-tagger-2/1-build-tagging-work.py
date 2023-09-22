import os
from pathlib import Path
import jinja2

# Read link.txt
with open("input.txt", "r", encoding="utf-8") as f:
    lines = f.readlines()

# Count lines and calculate padding for zero filling
total_lines = len(lines)
padding = len(str(total_lines))

templates = {f.name: f.read_text(encoding="utf-8") for f in Path("templates/tagging").glob("*.jinja")}

# Create directories and files
for idx, line in enumerate(lines, 1):
    url,notes = line.split(",", 1)
    folder_name = f"work/item-{str(idx).zfill(padding)}"
    work_item_dir = Path(folder_name)
    work_item_dir.mkdir(parents=True, exist_ok=True)

    with open(work_item_dir / "request.rest", "w", encoding="utf-8") as f:
        content = jinja2.Template(templates["request.rest.jinja"]).render()
        f.write(content)  # Your REST request here

    with open(work_item_dir / "request.body.txt", "w", encoding="utf-8") as f:
        content = jinja2.Template(templates["request.body.txt.jinja"]).render(
            url=url,
            notes=notes
        )
        f.write(content)
