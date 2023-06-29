import os
import glob
import cv2
import torch
import numpy as np
import torchvision.models as models
import matplotlib.pylab as plt
from PIL import Image
from sentence_transformers import SentenceTransformer, util
from sklearn.metrics.pairwise import cosine_similarity

from server.utils.tensors import to_tensors
from server.utils.vision import convert2pill, get_image_embedding

print("Loading CLIP Model...")
transformer_model = SentenceTransformer("clip-ViT-B-32")


torch_model = models.resnet50(pretrained=True)


def get_hist_acc(img1, img2):
    # img1 = cv2.imread(path1)
    imgs = [img1, img2]
    hists = []
    for i, img in enumerate(imgs):
        hsv = cv2.cvtColor(img, cv2.COLOR_BGR2HSV)
        hist = cv2.calcHist([hsv], [0, 1], None, [180, 256], [0, 180, 0, 256])
        cv2.normalize(hist, hist, 0, 1, cv2.NORM_MINMAX)
        hists.append(hist)

    ret = cv2.compareHist(hists[0], hists[1], cv2.HISTCMP_BHATTACHARYYA)
    return 1 - ret


def get_model_acc(img, img2):
    pass


def get_torch_acc(masked_image1, masked_image2):
    torch_model.eval()

    with torch.no_grad():
        im = Image.fromarray(masked_image1)
        im.save("masked_image1.jpeg")

        im = Image.fromarray(masked_image2)
        im.save("masked_image2.jpeg")

        print("==================")
        masked_image1 = to_tensors(masked_image1)
        masked_image2 = to_tensors(masked_image2)

        masked_image1 = (masked_image1 * 255).byte()
        masked_image2 = (masked_image2 * 255).byte()

        features1 = torch_model(torch.unsqueeze(masked_image1, 0).float())
        features2 = torch_model(torch.unsqueeze(masked_image2, 0).float())

        features1_norm = torch.nn.functional.normalize(features1, p=2, dim=1)
        features2_norm = torch.nn.functional.normalize(features2, p=2, dim=1)
        cosine_similarity = torch.mm(features1_norm, features2_norm.t())

        # euclidean_dist = torch.dist(features1, features2)
        # distance = euclidean_dist.item()

        print(f"distance: {cosine_similarity}")


def get_transformer_acc(image1, image2):
    image1 = convert2pill(image1)
    image2 = convert2pill(image2)
    encoded_image = transformer_model.encode(
        [image1, image2], batch_size=128, convert_to_tensor=True, show_progress_bar=True
    )
    processed_images = util.paraphrase_mining_embeddings(encoded_image)
    threshold = 0.99
    near_duplicates = [image for image in processed_images if image[0] < threshold]
    for score, image_id1, image_id2 in near_duplicates:
        # print("\nScore: {:.3f}%".format(score * 100))
        
        return score
    

def get_all_transformer_acc(reference_image, candidate_image_infos):
    reference_embedding = get_image_embedding(reference_image)

    candidate_embeddings = []
    candidate_keys = []
    for candidate_image_info in candidate_image_infos:
        masked_image, key = candidate_image_info
        candidate_keys.append(key)
        candidate_embedding = get_image_embedding(masked_image)
        candidate_embeddings.append(candidate_embedding)


    similarities = cosine_similarity([reference_embedding], candidate_embeddings)
    percent_similarities = (similarities + 1) * 50

    sorted_indices = np.argsort(percent_similarities)[0][::-1]
    sorted_percent_similarities = percent_similarities[0][sorted_indices]

    sorted_maps = [{'acc': sorted_percent_similarities[i], 'key': candidate_keys[indice_index]} for i, indice_index in enumerate(sorted_indices)]
    
    return sorted_maps

def get_ensemble_acc(image1, image2):
    # hist_acc = get_hist_acc()
    transformer_acc = get_transformer_acc(image1, image2)
    return transformer_acc
