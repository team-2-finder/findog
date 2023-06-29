import cv2, numpy as np
import matplotlib.pylab as plt


def get_hist_acc(img1, path2):
    # img1 = cv2.imread(path1)
    img2 = cv2.imread(path2)
    imgs = [img1, img2]
    hists = []
    for i, img in enumerate(imgs):
        hsv = cv2.cvtColor(img, cv2.COLOR_BGR2HSV)
        hist = cv2.calcHist([hsv], [0, 1], None, [180, 256], [0, 180, 0, 256])
        cv2.normalize(hist, hist, 0, 1, cv2.NORM_MINMAX)
        hists.append(hist)

    ret = cv2.compareHist(hists[0], hists[1], cv2.HISTCMP_BHATTACHARYYA)
    return 1 - ret
