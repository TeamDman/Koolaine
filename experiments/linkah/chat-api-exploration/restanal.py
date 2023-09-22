import json
import os

def parse_rest_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()

    requests = content.split('###')
    requests = [req.strip() for req in requests if req.strip()]
    
    parsed_requests = []
    
    for request in requests:
        lines = request.split('\n')
        
        # Skip comments
        lines = [line for line in lines if not line.startswith("#")]

        method, url = lines[0].split(' ')
        
        headers = {}
        body = {}
        
        for i in range(1, len(lines)):
            line = lines[i].strip()
            if not line:
                continue
            
            if line.startswith('{'):
                body = {
                    "type": "json",
                    "content": json.loads('\n'.join(lines[i:]))
                }
                break
            
            if line.startswith('< '):
                # This is a file to load for the body
                file_path = line[2:].strip()
                body = {
                    "type": "file",
                    "path": os.path.join(os.path.dirname(filepath), file_path)
                }
                break
            
            key, value = line.split(':', 1)
            headers[key.strip()] = value.strip()
        
        parsed_requests.append({
            'method': method,
            'url': url,
            'headers': headers,
            'body': body
        })

    return parsed_requests

# Test the function
import sys
filepath = sys.argv[1]
print(json.dumps(parse_rest_file(filepath), indent=4))
