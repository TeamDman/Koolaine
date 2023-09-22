import requests
import json
from pathlib import Path
import sys

def run_request(request_payload, n, work_dir):
    url = request_payload['url']
    headers = request_payload['headers']
    body_file = request_payload['body']['path']
    
    with open(body_file,"r") as f:
        body_content = json.load(f)

    response = requests.post(url, headers=headers, json=body_content)
    headers = "".join([f"{k}: {v}\n" for k, v in response.headers.items()])
    response_text = f"""HTTP/1.0 {response.status_code} {response.reason}
{headers}
{response.text}
"""

    response_file = work_dir / f"request-{n}.response"
    with response_file.open('w') as f:
        f.write(response_text)

def main(work_dir):
    work_dir = Path(work_dir)
    
    # Read requests.json
    json_path = Path("requests.json")
    with json_path.open('r') as f:
        requests_list = json.load(f)

    from tqdm import tqdm
    for n, request_payload in tqdm(enumerate(requests_list), total=len(requests_list)):
        run_request(request_payload, n, work_dir)

if __name__ == "__main__":
    # work_dir = sys.argv[1]
    work_dir = "work"
    main(work_dir)
