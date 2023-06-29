from sqlalchemy import select
from server.utils.preprocess import get_crops
from server.db import Database, model
from server.utils.acc import get_hist_acc
from PIL import Image
from fastapi import FastAPI

app = FastAPI(root_path="/ai")

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
        try:
            acc = get_hist_acc(img, path)
            res.append({"acc": acc, "key": key})
        except:
            continue
    res = sorted(res, key=lambda x: x["acc"], reverse=True)
    return {"results": res}


@app.on_event("startup")
async def startup():
    global mask_paths
    await Database.init()
    dogs = await get_all_paths()
    paths = [(dog.image_path, dog.desertion_no) for dog in dogs]
    image_paths = [dog.image_path for dog in dogs]

    for i in range(len(paths) / 100):
        paths_now = paths[i * 100 : (i + 1) * 100]
        image_paths_now = image_paths[i * 100 : (i + 1) * 100]
        images = get_crops(image_paths_now)

        try:
            for ((path, key), image) in zip(paths_now, images):
                path = "".join(path.split(".")[:-1])
                path = f"{path}-mask.jpg"
                mask_paths.append((path, key))
                Image.fromarray(image).save(path)
        except Exception as e:
            print(e)