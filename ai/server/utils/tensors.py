import torch
import torchvision.models as models
import torchvision.transforms as transforms
from PIL import Image

preprocess = transforms.Compose([
    transforms.Resize(256),
    transforms.CenterCrop(224),
    transforms.ToTensor()
])

def to_tensors(img):
    tensor = torch.from_numpy(img)
    tensor = tensor.permute(2, 0, 1)
    return tensor