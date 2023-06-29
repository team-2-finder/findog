from sqlalchemy import select
from server.utils.preprocess import get_crops
from server.db import Database, model
from server.utils.acc import get_hist_acc
from PIL import Image
from fastapi import FastAPI
import cv2

app = FastAPI(root_path="/ai")

mask_paths = []
batch_size = 1000
pre_imgs = []

async def get_all_paths() -> list[model.Dogs]:
    async with Database.async_session() as session:
        stmt = select(model.Dogs)
        result = await session.execute(stmt)
        dogs = result.scalars().all()
        return dogs


@app.get("/acc")
async def read_item(path: str):
    res = []
    img_ipt = cv2.imread(path)
    for (img, key) in pre_imgs:
        try:
            acc = get_hist_acc(img, img_ipt)
            res.append({"acc": acc, "key": key})
        except Exception as e:
            print(e)
            continue
    res = sorted(res, key=lambda x: x["acc"], reverse=True)
    print(res)
    return {"results": res}


@app.on_event("startup")
async def startup():
    global mask_paths
    global pre_imgs

    await Database.init()
    dogs = await get_all_paths()
    paths = [(dog.image_path, dog.desertion_no) for dog in dogs]
    image_paths = [dog.image_path for dog in dogs]

    for i in range(len(paths) // batch_size + 1):
        paths_now = paths[i * batch_size : (i + 1) * batch_size]
        image_paths_now = image_paths[i * batch_size : (i + 1) * batch_size]
        images = get_crops(image_paths_now)

        try:
            for ((path, key), image) in zip(paths_now, images):
                path = ".".join(path.split(".")[:-1])
                path = f"{path}-mask.jpg"
                mask_paths.append((path, key))
                Image.fromarray(image).save(path)
                pre_imgs.append((cv2.imread(path), key))

        except Exception as e:
            print(e)
        