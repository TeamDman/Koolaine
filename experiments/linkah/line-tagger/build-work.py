from pathlib import Path
import json
import sys

def write_to_rest_file(requests_list, rest_file_path):
    with open(rest_file_path, 'w') as rest_file:
        for idx, request_payload in enumerate(requests_list):
            method = request_payload.get('method', '')
            url = request_payload.get('url', '')
            headers = request_payload.get('headers', {})
            body = request_payload.get('body', {})
            
            # Write the method and URL
            rest_file.write(f"{method} {url}\n")
            
            # Write headers
            for header, value in headers.items():
                rest_file.write(f"{header}: {value}\n")
            
            # Write body
            if body.get('type') == 'file':
                rest_file.write(f"< {body.get('path')}\n")
            else:
                rest_file.write(f"{body}\n")
            
            # Write prompt and response files
            with open(f"prompt-{idx}.txt", 'w') as prompt_file:
                prompt_file.write(json.dumps(request_payload, indent=4))
            
            with open(f"prompt-{idx}-response.txt", 'w') as response_file:
                # Placeholder: You can populate this file with the actual response later
                response_file.write("Response will be saved here.")
            
            # Separate each request with '###'
            rest_file.write("###\n")

def process_line(line, n, work_dir):
    request_payload = {
        "method": "POST",
        "url": "http://127.0.0.1:5000/api/v1/generate",
        "headers": {
            "Content-Type": "application/json"
        },
        "body": {
            "type": "file",
            "path": f"{work_dir}/request-{n}.body.json"
        }
    }

    import jinja2
    url, notes = line.split(",", 1)
    prompt = jinja2.Template(open('prompt-template.jinja').read()).render(
        url=url,
        notes=notes
    )
    body = {
        "prompt": prompt,
        "max_new_tokens": 50,
        "_url": line.split(",")[0],
        "_notes": line.split(",", 1)[1],
    }

    # Create the JSON file
    json_path = work_dir / f"request-{n}.body.json"
    with json_path.open('w') as f:
        json.dump(body, f, indent=4)

    # Create the prompt text file
    txt_path = work_dir / f"request-{n}.body.prompt.txt"
    with txt_path.open('w') as f:
        f.write(body['prompt'])

    return request_payload

def main(input_file_path, work_dir):
    # Ensure the work directory exists
    work_dir = Path(work_dir)
    work_dir.mkdir(parents=True, exist_ok=True)

    with open(input_file_path, 'r') as f:
        lines = f.readlines()

    requests_list = []
    for n, line in enumerate(lines):
        line = line.strip()
        if not line:
            continue
        requests_list.append(process_line(line, n, work_dir))

    # Create the JSON file
    json_path = Path("requests.json")
    with json_path.open('w') as f:
        json.dump(requests_list, f, indent=4)

    # Create the .rest file
    rest_path = Path("requests.rest")
    with rest_path.open('w') as f:
        for n, req in enumerate(requests_list):
            f.write(f"{req['method']} {req['url']}\n")
            for key, value in req['headers'].items():
                f.write(f"{key}: {value}\n\n")
            f.write(f"< {req['body']['path']}\n")
            f.write("###\n")

if __name__ == "__main__":
    input_file_path = sys.argv[1]
    work_dir = sys.argv[2]
    main(input_file_path, work_dir)
