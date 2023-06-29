from sqlalchemy import select
from server.utils.preprocess import get_crops
from server.db import Database, model
from server.utils.acc import get_hist_acc
from PIL import Image
from fastapi import FastAPI

app = FastAPI()

mask_paths = []

async def get_all_paths() -> list[model.Dogs]:
    async with Database.async_session() as session:
        stmt = select(model.Dogs)
        result = await session.execute(stmt)
        dogs = result.scalars().all()
        return dogs


@app.get("/acc")
async def read_item(path: str):
    res = []
    for (img, key) in mask_paths:
        acc = get_hist_acc(img, path)
        res.append({"acc": acc, "key": key})
    return {"results": res}


@app.on_event("startup")
async def startup():
    global mask_paths
    await Database.init()
    dogs = await get_all_paths()
    paths = [(dog.image_path, dog.desertion_no) for dog in dogs]
    images = get_crops([f"../server/{dog.image_path}" for dog in dogs])

    for (path, key) in paths:
        path = "".join(path.split(".")[:-1])
        path = f"{path}-mask.jpg"
        mask_paths.append((path, key))

    for ((path, _), image) in zip(mask_paths, images):
        Image.fromarray(image).save(path)
