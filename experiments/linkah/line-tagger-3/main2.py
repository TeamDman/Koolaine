from tqdm import tqdm
from time import sleep

def process_list_with_progress_bar(items, func, *args, **kwargs):
    failures = []
    results = []
    with tqdm(total=len(items)) as pbar:
        for item in items:
            pbar.set_description(f"Processing {item[0]}")
            try:
                result = func(item, *args, **kwargs)
                results.append(result)
            except Exception as e:
                failures.append((item, e))
                results.append(None)
            pbar.update(1)
            sleep(1)  # simulate delay
    return results, failures

def fetch_webpage_content(item):
    url, notes = item
    import requests
    from bs4 import BeautifulSoup
    page = requests.get(url)
    soup = BeautifulSoup(page.content, 'html.parser')
    innerText = soup.get_text()
    return (url, innerText)

if __name__ == '__main__':
    with open("input.txt","r", encoding="utf-8") as f:
        lines = [line.strip().split(",", 1) for line in f.readlines()]

    results, failures = process_list_with_progress_bar(lines, fetch_webpage_content)

    with open("output2.txt","w", encoding="utf-8") as f:
        for url, innerText in results:
            f.write(f"{url}\n########\n{innerText}\n########\n")
