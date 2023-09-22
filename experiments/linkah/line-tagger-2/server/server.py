from fastapi import FastAPI, Request
import requests
import json

app = FastAPI()

@app.post("/echo")
async def echo(request: Request):
    body = await request.body()
    headers = dict(request.headers)
    return {
        "body": body.decode("utf-8"),
        "headers": headers
    }

@app.post("/generate")
async def generate(request: Request):
    text_body = await request.body()
    text_body = text_body.decode("utf-8")

    # Transform body
    json_body = {
        "prompt": text_body,
        "max_new_tokens": 50
    }

    # Forward request
    response = requests.post(
        "http://127.0.0.1:5000/api/v1/generate",
        headers={"Content-Type": "application/json"},
        json=json_body
    )

    return response.json()
