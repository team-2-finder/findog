import torch
import torchvision.transforms.functional as F
from torchvision.models.segmentation import fcn_resnet50, FCN_ResNet50_Weights
from torchvision.io import read_image
import torchvision.transforms as transforms
import numpy as np
from PIL import Image

preprocess = transforms.Compose(
    [
        transforms.Resize(256),
        transforms.CenterCrop(224),
        transforms.ToTensor(),
        transforms.Normalize(mean=[0.485, 0.456, 0.406], std=[0.229, 0.224, 0.225]),
    ]
)


def preprocess_image(image_path):
    image = Image.open(image_path).convert("RGB")
    image = preprocess(image).unsqueeze(0)
    return image


weights = FCN_ResNet50_Weights.DEFAULT
transform_func = weights.transforms(resize_size=None)


def get_crops(paths):
    try:
        with torch.no_grad():
            min_height = min_width = 1e9

            dog_list = []
            cnt = 0
            for path, key in paths:
                try:
                    dog_int = read_image(path)
                    min_height = min(min_height, dog_int.shape[1])
                    min_width = min(min_width, dog_int.shape[2])
                    dog_list.append((dog_int, (path, key)))
                    cnt += 1
                    if cnt == 200:
                        break
                except:
                    pass

            for i in range(len(dog_list)):
                dog_int = dog_list[i][0]
                dog_int = F.resize(dog_int, (min_height, min_width))
                dog_list[i] = (dog_int, dog_list[i][1])

            model = fcn_resnet50(weights=weights, progress=False)
            model = model.eval()

            batch = torch.stack([transform_func(d) for d, _ in dog_list])
            output = model(batch)["out"]
            print(output.shape, output.min().item(), output.max().item())

            sem_class_to_idx = {
                cls: idx for (idx, cls) in enumerate(weights.meta["categories"])
            }
            normalized_masks = torch.nn.functional.softmax(output, dim=1)

            dog_masks = [
                normalized_masks[img_idx, sem_class_to_idx["dog"]]
                for img_idx in range(len(dog_list))
            ]

            masked_imgs = []

            for idx, ((dog_img, key), dog_mask) in enumerate(zip(dog_list, dog_masks)):
                np_img = dog_img.numpy()
                np_mask = dog_mask.detach().numpy()

                masked_img = (
                    np_img.copy()
                )  # Initialize masked image array as a copy of original image

                for c in range(np_img.shape[0]):
                    masked_img[c] = np.where(
                        np_mask > 0.01, np_img[c], 0
                    )  # Apply the mask to the corresponding channel
                np_masked = np.transpose(
                    masked_img, (1, 2, 0)
                )  # Convert shape to (H, W, C)

                masked_imgs.append((np_masked, key))

            return masked_imgs
    except Exception as e:
        print(e)
