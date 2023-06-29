from utils.acc import get_hist_acc
from fastapi import FastAPI

app = FastAPI()

@app.get("/acc")
async def read_item(path: str):
    results = get_hist_acc(path)
    return {"results": results}