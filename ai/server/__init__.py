from sqlalchemy import select
from server.db import Database, model
from server.utils.acc import get_hist_acc
from fastapi import FastAPI

app = FastAPI()


async def get_all_paths() -> list[model.Dogs]:
    async with Database.async_session() as session:
        stmt = select(model.Dogs)
        result = await session.execute(stmt)
        dogs = result.scalars().all()
        return dogs


@app.get("/acc")
async def read_item(path: str):
    results = get_hist_acc(path)
    return {"results": results}


@app.on_event("startup")
async def startup():
    await Database.init()
