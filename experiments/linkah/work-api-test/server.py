from fastapi import FastAPI, Request

app = FastAPI()

@app.post("/echo")
async def echo(request: Request):
    body = await request.body()
    headers = dict(request.headers)
    return {
        "body": body.decode("utf-8"),
        "headers": headers
    }
