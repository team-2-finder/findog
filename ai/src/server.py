from fastapi import FastAPI

app = FastAPI()

@app.check("/")
async def read_root():
    pass