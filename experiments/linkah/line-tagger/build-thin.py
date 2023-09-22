from pathlib import Path
import json
if __name__ == "__main__":
    outfile = Path("thin.json")
    infile = Path("responses.json")

    # output_thin_dict = {v["url"]:v["response"]["Body"]["results"][0]["text"].strip() for k,v in output_dict.items()}
    # with output_thin_json_path.open('w') as f:
    #     json.dump(output_thin_dict, f, indent=4)

    with infile.open('r') as f:
        responses = json.load(f)
    
    outdata = []
    for v in responses.values():
        prompt = v["request_body"]["prompt"]
        generated = v["response"]["Body"]["results"][0]["text"]
        prompt_completed = prompt + generated
        aijson = prompt_completed.split("### Response:",1)[1]
        try:
            tags = json.loads(aijson)
        except:
            tags = ["ERROR",*aijson.split(",")]
        outdata.append({
            "url": v["request_body"]["_url"],
            "tags": tags,
        })
    with outfile.open('w') as f:
        json.dump(outdata, f, indent=4)
