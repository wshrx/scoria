#!/usr/bin/env python3
"""Crop real in-game Minecraft captures into the launcher art.

Sources are the Scoria/Horizon test renders in ~/Pictures:
  horizon-visuals-final-compare.png   day beach / ocean
  horizon-visuals-day-compare.png     forest / stone build
  horizon-visuals-ingame-compare.png  night beach
The tutorial box and HUD are cropped out. No generated imagery.
"""
import os
import numpy as np
from PIL import Image

HERE = os.path.dirname(__file__)
OUT = os.path.join(HERE, "art")
SRC = "/home/shrx/Pictures"


def crop(name, box, size):
    im = Image.open(os.path.join(SRC, name)).convert("RGB")
    im = im.crop(box).resize(size, Image.LANCZOS)
    return im


def grade(img, gain=(1, 1, 1), lift=(0, 0, 0), contrast=1.0, sat=1.0):
    a = np.asarray(img).astype(np.float32) / 255.0
    a = a * np.array(gain, dtype=np.float32) + np.array(lift, dtype=np.float32)
    a = (a - 0.5) * contrast + 0.5
    lum = a @ np.array([0.2126, 0.7152, 0.0722], dtype=np.float32)
    a = lum[..., None] + (a - lum[..., None]) * sat
    return np.clip(a, 0, 1)


def vignette(a, strength=0.30):
    h, w, _ = a.shape
    yy, xx = np.mgrid[0:h, 0:w]
    d = np.sqrt(((xx - w / 2) / (w / 2)) ** 2 + ((yy - h / 2) / (h / 2)) ** 2)
    return a * np.clip(1 - (d / 1.45) ** 2 * strength, 0, 1)[..., None]


def grain(a, seed, amount=0.008):
    rng = np.random.default_rng(seed)
    return np.clip(a + rng.normal(0, amount, a.shape), 0, 1)


def save(a, name, seed):
    a = grain(vignette(a, 0.32), seed)
    Image.fromarray((a * 255).astype(np.uint8)).save(os.path.join(OUT, name),
                                                    optimize=True)


def main():
    # 16:9 crops from the right-hand (shader) panel, between tutorial and HUD
    box = (1280, 90, 2115, 560)          # 835 x 470
    size = (1024, 576)

    new = crop("horizon-visuals-final-compare.png", box, size)
    save(grade(new, (1.06, 0.97, 0.90), (0.0, 0.0, 0.0), 1.05, 1.10),
         "thumb-new-world.png", 12)

    build = crop("horizon-visuals-day-compare.png", box, size)
    save(grade(build, (0.90, 1.00, 1.08), (0.0, 0.0, 0.01), 1.04, 0.95),
         "thumb-build.png", 21)

    hard = crop("horizon-visuals-ingame-compare.png", box, size)
    save(grade(hard, (1.18, 0.80, 0.72), (0.0, 0.0, 0.0), 1.16, 0.84),
         "thumb-hardcore.png", 31)

    # portrait hero for the D poster, 5:6
    hero = crop("horizon-visuals-final-compare.png", (1440, 90, 1836, 560),
                (944, 1120))
    save(grade(hero, (1.05, 0.95, 0.87), (0.0, 0.0, 0.0), 1.08, 1.08),
         "hero-new-world.png", 11)

    # 1:1 light hero for E, beach scene
    soft = crop("horizon-visuals-final-compare.png", (1350, 90, 1830, 570),
                (1024, 1024))
    save(grade(soft, (1.08, 1.04, 0.98), (0.06, 0.06, 0.05), 0.95, 0.86),
         "hero-soft.png", 51)

    print("written", sorted(f for f in os.listdir(OUT) if f.endswith(".png")))


if __name__ == "__main__":
    main()
