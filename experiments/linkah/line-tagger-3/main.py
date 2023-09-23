from tqdm import tqdm
from time import sleep
with open("input.txt","r", encoding="utf-8") as f:
    lines = f.readlines()
failures = []
with open("output.txt","w", encoding="utf-8") as f:
    f.truncate(0)
    with tqdm(total=len(lines)) as pbar:
        for line in lines:
            url, notes = line.split(",",1)
            pbar.set_description(f"Processing {url}")
            # append url
            f.write(url)
            f.write("\n")

            try:
                # Get the innerText of the page
                import requests
                from bs4 import BeautifulSoup
                page = requests.get(url)
                soup = BeautifulSoup(page.content, 'html.parser')
                innerText = soup.get_text()
            except Exception as e:
                failures.append((url,notes,e))
                innerText = "ERROR"

            f.write(f"########\n{innerText}\n########\n")

            pbar.update(1)
            sleep(1)
