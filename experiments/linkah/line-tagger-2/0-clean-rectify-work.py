from pathlib import Path
from tqdm import tqdm

work_dir = Path("work")
for item in tqdm(list(work_dir.glob("item-*"))):
    rec = item / "rectify"
    if rec.exists():
        for f in tqdm(list(rec.glob("*"))):
            f.unlink()
        rec.rmdir()
