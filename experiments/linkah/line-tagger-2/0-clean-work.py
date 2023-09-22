from pathlib import Path
from tqdm import tqdm

work_dir = Path("work")
if work_dir.exists():
    for f in tqdm(list(work_dir.glob("*"))):
        f.unlink()
    work_dir.rmdir()
