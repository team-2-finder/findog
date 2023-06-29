import os
import requests
def download():
    pt_url = 'https://openaipublic.azureedge.net/clip/models/40d365715913c9da98579312b702a82c18be219cc2a73407c4526f58eba950af/ViT-B-32.pt'
    cache_dir = os.path.expanduser('/model/clip')
    os.makedirs(cache_dir, exist_ok=True)

    file_path = os.path.join(cache_dir, 'ViT-B-32.pt')

    if os.path.exists(file_path):
        return

    response = requests.get(pt_url, verify=False)
    with open(file_path, 'wb') as file:
        file.write(response.content)
        

download()

from typing import List
from sqlalchemy import select
from server.utils.preprocess import get_crops
from server.db import Database, model

from server.utils.acc import get_ensemble_acc, get_all_transformer_acc, get_hist_acc
from PIL import Image
from fastapi import FastAPI
import cv2

app = FastAPI(root_path="/ai")

mask_paths = []
batch_size = 1000
pre_imgs = []

async def get_all_paths() -> List[model.Dogs]:
    async with Database.async_session() as session:
        stmt = select(model.Dogs)
        result = await session.execute(stmt)
        dogs = result.scalars().all()
        return dogs


@app.get("/acc")
async def read_item(path: str):
    res = []
    img_ipt = cv2.imread(path)
    
    masked_input = get_crops([path])[0]
    # all_transformer_res = get_all_transformer_acc(img_ipt, pre_imgs)
    # print(all_transformer_res)

    for img, key in pre_imgs:
        try:
            acc = get_hist_acc(masked_input, img_ipt)
            res.append({"acc": acc, "key": key})
        except Exception as e:
            print(e)
            continue
    res = sorted(res, key=lambda x: x["acc"], reverse=True)
    
    return {"results": res}

@app.on_event("startup")
async def startup():
    global mask_paths
    global pre_imgs

    await Database.init()
    dogs = await get_all_paths()
    paths = [(dog.image_path, dog.desertion_no) for dog in dogs]

    for i in range(len(paths) // batch_size + 1):
        paths_now = paths[i * batch_size : (i + 1) * batch_size]
        images = get_crops(paths_now)

        try:
            for image, (path, key) in images:
                path = ".".join(path.split(".")[:-1])
                path = f"{path}-mask.jpg"
                mask_paths.append((path, key))
                Image.fromarray(image).save(path)
                pre_imgs.append((cv2.imread(path), key))

        except Exception as e:
            print(e)
