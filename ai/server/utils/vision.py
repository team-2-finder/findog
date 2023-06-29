import torch
import torchvision.transforms as transforms
import numpy as np
import cv2
from PIL import Image
import clip
from sklearn.metrics.pairwise import cosine_similarity
import os

device = "cuda" if torch.cuda.is_available() else "cpu"
model, preprocess = clip.load('~/.cache/clip/ViT-B-32.pt')

from PIL import Image
def convert2pill(image):
    image_rgb = cv2.cvtColor(image, cv2.COLOR_BGR2RGB)
    image_pil = Image.fromarray(image_rgb)
    return image_pil

def get_image_embedding(image):
    image = Image.fromarray(image)
    image = preprocess(image).unsqueeze(0).to(device)
    with torch.no_grad():
        image_embedding = model.encode_image(image)
    return image_embedding.cpu().numpy().flatten()