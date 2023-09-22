import re
import sys
from typing import List, Dict


def parse_request(filepath: str) -> List[Dict]:
    with open(filepath, "r") as f:
        content = f.read()
        print(f"Found {len(content)} bytes in {filepath}")
        requests = re.findall("^.*\n(\w+)\s*:\s*((?P<get>.*)|(?P<post>.+))\s+(\S+)", content)
        results = []
        for request in requests:
            if request[0].startswith('GET'):
                url, headers, body = request
                results.append({
                    'method': 'GET',
                    'location': url,
                    'headers': {header.strip(): value for header, value in zip(headers.splitlines(), headers.split())},
                    'body': body.strip()
                })
            else:
                url, headers, json_payload = request
                results.append({
                    'method': 'POST',
                    'location': url,
                    'headers': {header.strip(): value for header, value in zip(headers.splitlines(), headers.split())},
                    'json_payload': json_payload.strip()
                })
        return results
    
if __name__ == "__main__":
    # read first param
    print(parse_request(sys.argv[1]))