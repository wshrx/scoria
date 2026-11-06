#!/usr/bin/env python3
"""Scoria interface directions, round 2.

A strict 32px margin / 4px grid, one Lucide-style icon set at 18px, and three
structural ideas instead of one shell with three palettes.

Usage: python3 build.py d|preparing|playing|states|e|all
"""
import json
import sys

W, H = 1280, 720
ART = "/home/shrx/dev/scoria/design/figcraft/art"

# ---------------------------------------------------------------- icons
# Lucide paths (24x24, stroke 1.75, round caps/joins), MIT.
ICONS = {
    "play": ["M5 4l14 8-14 8z"],
    "grid": ["M3 3h7v7H3z", "M14 3h7v7h-7z", "M14 14h7v7h-7z", "M3 14h7v7H3z"],
    "globe": ["M12 2a10 10 0 1 0 0 20 10 10 0 0 0 0-20z", "M2 12h20",
              "M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"],
    "server": ["M4 4h16v6H4z", "M4 14h16v6H4z", "M8 7h.01", "M8 17h.01"],
    "package": ["M12 3l9 5v8l-9 5-9-5V8z", "M3 8l9 5 9-5", "M12 13v8"],
    "settings": ["M4 6h10", "M18 6h2", "M4 12h4", "M12 12h8", "M4 18h12", "M20 18h0"],
    "search": ["M11 4a7 7 0 1 0 0 14 7 7 0 0 0 0-14z", "M20 20l-4.3-4.3"],
    "x": ["M6 6l12 12", "M18 6L6 18"],
    "minus": ["M5 12h14"],
    "square": ["M5 5h14v14H5z"],
    "arrow": ["M5 12h14", "M13 6l6 6-6 6"],
    "chevron": ["M9 6l6 6-6 6"],
    "menu": ["M4 7h16", "M4 12h16", "M4 17h16"],
    "download": ["M12 3v12", "M7 11l5 5 5-5", "M4 21h16"],
    "check": ["M4 12l5 5L20 6"],
    "plus": ["M12 5v14", "M5 12h14"],
    "user": ["M12 12a4 4 0 1 0 0-8 4 4 0 0 0 0 8z", "M4 21a8 8 0 0 1 16 0"],
    "folder": ["M3 7h7l2 2h9v10H3z"],
    "zap": ["M13 2L4 14h7l-1 8 9-12h-7z"],
    "refresh": ["M3 12a9 9 0 1 0 3-6.7L3 8", "M3 3v5h5"],
    "clock": ["M12 2a10 10 0 1 0 0 20 10 10 0 0 0 0-20z", "M12 6v6l4 2"],
    "map-pin": ["M20 10c0 4.993-5.539 10.193-7.399 11.799a1 1 0 0 1-1.202 0C9.539 20.193 4 14.993 4 10a8 8 0 0 1 16 0", "M12 10a2 2 0 1 0 0-4 2 2 0 0 0 0 4z"],
    "alert": ["M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z", "M12 9v4", "M12 17h.01"],
    "hard-drive": ["M22 12H2", "M5.45 5.11L2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z", "M6 16h.01", "M10 16h.01"],
    "arrow-up-right": ["M7 17L17 7", "M7 7h10v10"],
    "activity": ["M22 12h-4l-3 9L9 3l-3 9H2"],
    "terminal": ["M4 17l6-6-6-6", "M12 19h8"],
    "layers": ["M12 2l9 5-9 5-9-5z", "M3 12l9 5 9-5", "M3 17l9 5 9-5"],
    "keyboard": ["M2 6h20v12H2z", "M6 10h.01", "M10 10h.01", "M14 10h.01", "M18 10h.01", "M6 14h.01", "M18 14h.01", "M10 14h4"],
    "help": ["M12 2a10 10 0 1 0 0 20 10 10 0 0 0 0-20z", "M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3", "M12 17h.01"],
    "shield-off": ["M19.69 14a6.9 6.9 0 0 0 .31-2V5l-8-3-3.16 1.18", "M4.73 4.73L4 5v7c0 6 8 10 8 10a20.29 20.29 0 0 0 5.62-4.38", "M2 2l20 20"],
    "users": ["M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2", "M9 7a4 4 0 1 0 0 8 4 4 0 0 0 0-8z", "M22 21v-2a4 4 0 0 0-3-3.87", "M16 3.13a4 4 0 0 1 0 7.75"],
}


# Visual ink bbox in the 24-unit viewBox, measured by icon_bbox.py.
ICON_BBOX = {
    "play": [
        4.25,
        3.25,
        15.5,
        17.5
    ],
    "grid": [
        2.25,
        2.25,
        19.5,
        19.5
    ],
    "globe": [
        1.25,
        1.25,
        21.5,
        21.5
    ],
    "server": [
        3.25,
        3.25,
        17.5,
        17.5
    ],
    "package": [
        2.25,
        2.25,
        19.5,
        19.5
    ],
    "settings": [
        3.25,
        5.25,
        17.5,
        13.5
    ],
    "search": [
        3.25,
        3.25,
        17.5,
        17.5
    ],
    "x": [
        5.25,
        5.25,
        13.5,
        13.5
    ],
    "minus": [
        4.25,
        11.25,
        15.5,
        1.5
    ],
    "square": [
        4.25,
        4.25,
        15.5,
        15.5
    ],
    "arrow": [
        4.25,
        5.25,
        15.5,
        13.5
    ],
    "chevron": [
        8.25,
        5.25,
        7.5,
        13.5
    ],
    "menu": [
        3.25,
        6.25,
        17.5,
        11.5
    ],
    "download": [
        3.25,
        2.25,
        17.5,
        19.5
    ],
    "check": [
        3.25,
        5.25,
        17.5,
        12.5
    ],
    "plus": [
        4.25,
        4.25,
        15.5,
        15.5
    ],
    "user": [
        3.25,
        3.25,
        17.5,
        18.5
    ],
    "folder": [
        2.25,
        6.25,
        19.5,
        13.5
    ],
    "zap": [
        3.25,
        1.25,
        16.5,
        21.5
    ],
    "refresh": [
        2.25,
        2.25,
        19.5,
        19.55
    ],
    "clock": [
        1.25,
        1.25,
        21.5,
        21.5
    ],
    "map-pin": [
        3.25,
        1.25,
        17.5,
        21.5
    ],
    "alert": [
        0.8,
        2.1,
        22.4,
        19.65
    ],
    "hard-drive": [
        1.25,
        3.25,
        21.5,
        17.5
    ],
    "arrow-up-right": [
        6.25,
        6.25,
        11.5,
        11.5
    ],
    "activity": [
        1.25,
        2.25,
        21.5,
        19.5
    ],
    "terminal": [
        3.25,
        4.25,
        17.5,
        15.5
    ],
    "layers": [
        2.25,
        1.25,
        19.5,
        21.5
    ],
    "keyboard": [
        1.25,
        5.25,
        21.5,
        13.5
    ],
    "help": [
        1.25,
        1.25,
        21.5,
        21.5
    ],
    "shield-off": [
        1.25,
        1.25,
        21.5,
        21.5
    ],
    "users": [
        1.25,
        2.35,
        21.5,
        19.4
    ]
}


def grad(stops, kind="linear", frm=(0, 0), to=(0, 1)):
    g = {"type": kind + "Gradient", "stops": stops}
    if frm is not None:
        g["from"] = list(frm)
    if to is not None:
        g["to"] = list(to)
    return g


class Scene:
    def __init__(self):
        self.cmds = []
        self.next_id = 2
        self.ids = {}
        self.stack = []

    def push(self, node):
        self.stack.append(node)

    def pop(self):
        self.stack.pop()

    def cur(self):
        return self.stack[-1] if self.stack else None

    def create(self, cmd, params, key=None):
        self.cmds.append({"command": cmd, "params": params})
        i = self.next_id
        self.next_id += 1
        if key:
            self.ids[key] = i
        return i

    # ------------------------------------------------------------ prims
    def frame(self, x, y, w, h, key=None, name=None, fill=None, gradient=None,
              radius=0, stroke=None, sw=1, clip=False, opacity=None, parent=None):
        if parent is None:
            parent = self.cur()
        props = {}
        if gradient:
            props["fills"] = [gradient]
        elif fill:
            props["fill"] = fill
        if radius:
            props["cornerRadius"] = radius
        if stroke:
            props["stroke"] = stroke
            props["strokeWeight"] = sw
            props["strokeAlign"] = "inside"
            if not fill and not gradient:
                props["fill"] = "none"
        if clip:
            props["clip"] = True
        if opacity is not None:
            props["opacity"] = opacity
        p = {"x": x, "y": y, "width": w, "height": h, "props": props}
        if name:
            p["name"] = name
        if parent:
            p["parent"] = parent
        return self.create("frame.create", p, key)

    def rect(self, x, y, w, h, key=None, name=None, fill=None, gradient=None,
             radius=0, stroke=None, sw=1, opacity=None, parent=None, clip=False,
             dashes=None):
        if parent is None:
            parent = self.cur()
        props = {}
        if gradient:
            props["fills"] = [gradient]
        elif fill:
            props["fill"] = fill
        if radius:
            props["cornerRadius"] = radius
        if stroke:
            props["stroke"] = stroke
            props["strokeWeight"] = sw
            props["strokeAlign"] = "inside"
            if not fill and not gradient:
                props["fill"] = "none"
        if opacity is not None:
            props["opacity"] = opacity
        if clip:
            props["clip"] = True
        p = {"x": x, "y": y, "width": w, "height": h, "props": props}
        if name:
            p["name"] = name
        if parent:
            p["parent"] = parent
        return self.create("shape.rectangle", p, key)

    def ellipse(self, x, y, w, h, key=None, name=None, fill=None, gradient=None,
                stroke=None, sw=1, opacity=None, parent=None):
        if parent is None:
            parent = self.cur()
        props = {}
        if gradient:
            props["fills"] = [gradient]
        elif fill:
            props["fill"] = fill
        if stroke:
            props["stroke"] = stroke
            props["strokeWeight"] = sw
            props["strokeAlign"] = "inside"
            if not fill and not gradient:
                props["fill"] = "none"
        if opacity is not None:
            props["opacity"] = opacity
        p = {"x": x, "y": y, "width": w, "height": h, "props": props}
        if name:
            p["name"] = name
        if parent:
            p["parent"] = parent
        return self.create("shape.ellipse", p, key)

    def text(self, s, x, y, size=14, family="Medeo Sans", style="Regular",
             color="#F7EFE7", width=None, height=None, align="left",
             valign="top", lh=None, ls=None, key=None, name=None, parent=None,
             opacity=None, features=None):
        if parent is None:
            parent = self.cur()
        props = {"fontFamily": family, "fontSize": size, "fontStyle": style,
                 "fill": color}
        if height is not None:
            if width is None:
                longest = max(s.split("\n"), key=len)
                factor = 0.68 if style in ("Bold", "SemiBold", "Medium", "ExtraBold") else 0.63
                width = max(24, int(len(longest) * size * factor) + 8)
            props["height"] = height
            props["textAutoResize"] = "none"
        if align != "left":
            props["textAlign"] = align
        if valign != "top":
            props["textAlignVertical"] = valign
        if lh is not None:
            props["lineHeight"] = lh
        if ls is not None:
            props["letterSpacing"] = ls
        if opacity is not None:
            props["opacity"] = opacity
        if features is not None:
            props["features"] = features
        p = {"text": s, "x": x, "y": y, "props": props}
        if width is not None:
            p["width"] = width
        if name:
            p["name"] = name
        if parent:
            p["parent"] = parent
        return self.create("text.create", p, key)

    def vector(self, x, y, w, h, path, key=None, name=None, fill=None, stroke=None,
               sw=1, dashes=None, opacity=None, parent=None, cap="round",
               join="round"):
        if parent is None:
            parent = self.cur()
        props = {}
        if fill:
            props["fill"] = fill
        if stroke:
            props["stroke"] = stroke
            props["strokeWeight"] = sw
            props["strokeCap"] = cap
            props["strokeJoin"] = join
            if not fill:
                props["fill"] = "none"
        if dashes:
            props["dashes"] = dashes
        if opacity is not None:
            props["opacity"] = opacity
        p = {"x": x, "y": y, "width": w, "height": h, "path": path, "props": props}
        if name:
            p["name"] = name
        if parent:
            p["parent"] = parent
        return self.create("vector.create", p, key)

    def image(self, path, x, y, w, h, key=None, name=None, parent=None,
              mode="fit"):
        if parent is None:
            parent = self.cur()
        p = {"path": path, "x": x, "y": y, "width": w, "height": h, "mode": mode}
        if name:
            p["name"] = name
        if parent:
            p["parent"] = parent
        return self.create("image.place", p, key)

    def icon(self, kind, x, y, size=18, color="#F7EFE7", parent=None, sw=1.5,
             filled=False):
        if kind == "minus":
            self.rect(x + 2, y + size / 2 - 0.7, size - 4, 1.4, fill=color,
                      parent=parent)
            return None
        path = "".join(ICONS[kind])
        bx, by, bw, bh = ICON_BBOX[kind]
        scale = size / max(bw, bh)
        vw, vh = bw * scale, bh * scale
        left = x + (size - vw) / 2.0
        top = y + (size - vh) / 2.0
        vx = left - bx * scale
        vy = top - by * scale
        if filled:
            return self.vector(vx, vy, vw, vh, path, fill=color, parent=parent)
        return self.vector(vx, vy, vw, vh, path, stroke=color, sw=sw,
                           parent=parent)

    def window_controls(self, y, color, x=1174):
        self.icon("minus", x, y, 14, color, sw=1.4)
        self.icon("square", x + 30, y, 14, color, sw=1.4)
        self.icon("x", x + 60, y, 14, color, sw=1.4)

    # ------------------------------------------------------------ shared

    def icon_text(self, kind, x, y, h, label, size=14, gap=8,
                  color="#9C948C", text_size=13, family="Medeo Sans",
                  style="Regular", text_color=None, width=None, align="left",
                  parent=None, features=None):
        """Icon + label on one 8 px rhythm, both vertically centred in h."""
        if parent is None:
            parent = self.cur()
        self.icon(kind, x, y + (h - size) / 2.0, size, color, parent=parent)
        self.text(label, x + size + gap, y, text_size, family, style,
                  text_color if text_color is not None else color,
                  width=width, height=h, valign="middle", align=align,
                  parent=parent, features=features)

    def world_thumb(self, x, y, w, h, colors, radius=8, letter=None,
                    letter_color="#FFFFFF10", parent=None, image=None):
        f = self.frame(x, y, w, h, radius=radius, clip=True, parent=parent)
        self.push(f)
        if image:
            self.image(image, 0, 0, w, h, parent=f, mode="fill")
            self.rect(0, 0, w, h, gradient=grad(["#00000000", "#00000055"]), parent=f)
        else:
            self.rect(0, 0, w, h, gradient=grad(colors), parent=f)
            self.vector(0, h * 0.55, w, h * 0.45,
                        f"M0 {h*0.45:.0f}L{w*0.18:.0f} {h*0.22:.0f}L{w*0.32:.0f} {h*0.38:.0f}L{w*0.48:.0f} {h*0.12:.0f}L{w*0.66:.0f} {h*0.34:.0f}L{w*0.82:.0f} {h*0.18:.0f}L{w:.0f} {h*0.40:.0f}V{h*0.45:.0f}H0Z",
                        fill="#FFFFFF12", parent=f)
        if letter:
            self.text(letter, 10, 4, h * 0.55, "Medeo Sans", "Bold",
                      letter_color, parent=f, lh="90%")
        self.pop()
        return f


# ================================================================ D
D = dict(
    bg="#0B0B0D", surface="#141416", surface2="#1A1A1D", border="#26262A",
    fg="#F4F0EA", muted="#9C948C", accent="#FF6A3D", accent_fg="#1A0A04",
    soft="#2A1710", success="#6FBF8B",
)


def direction_d(sc, ox=0, oy=0, state="ready", name="D Signal"):
    s = D
    root = sc.frame(ox, oy, W, H, key=f"d_{state}", name=name, fill=s["bg"], clip=True)
    sc.push(root)
    sc.ellipse(600, -220, 900, 900,
               gradient=grad(["#3A1B10", "#0B0B0D00"], "radial", (0.5, 0.5), (1, 1)),
               parent=root)
    sc.rect(0, 56, W, 1, fill="#FFFFFF08", parent=root)

    # top bar
    sc.image("/home/shrx/dev/scoria/launcher/ui/assets/scoria-mark.png", 32, 18, 20, 20)
    sc.text("Scoria", 60, 16, 14, "Medeo Sans", "SemiBold", s["fg"], height=24,
            valign="middle")
    sc.text("0.1.2", 112, 18, 10, "Medeo Mono", "Regular", s["muted"], height=20,
            valign="middle")
    nav = [("play", "Играть", 528, 56, True), ("package", "Моды", 608, 48, False),
           ("globe", "Серверы", 680, 72, False)]
    for ic, label, x, w, on in nav:
        col = s["fg"] if on else s["muted"]
        sc.icon_text(ic, x, 16, 24, label, size=14, color=col,
                     text_size=13, style="SemiBold" if on else "Regular",
                     width=w)
    sc.rect(550, 50, 44, 2, fill=s["accent"], radius=1)
    sc.rect(800, 12, 220, 32, fill="#141416", radius=8, stroke=s["border"], sw=1)
    sc.icon_text("search", 816, 12, 32, "Поиск миров и модов", size=14,
                 color=s["muted"], text_size=11, width=118)
    sc.text("Ctrl K", 968, 12, 9, "Medeo Mono", "Regular", s["muted"],
            width=44, height=32, valign="middle", align="right")
    sc.rect(1040, 16, 72, 24, fill="#141416", radius=6)
    sc.ellipse(1050, 25, 6, 6, fill=s["success"])
    sc.text("Офлайн", 1062, 16, 10, "Medeo Sans", "Medium", s["muted"],
            width=44, height=24, valign="middle")
    sc.icon("settings", 1130, 20, 16, s["muted"])
    sc.window_controls(22, s["muted"])

    # left column
    eyebrow = {"ready": "ПОСЛЕДНИЙ ЗАПУСК · 2 Ч НАЗАД",
               "preparing": "ПОДГОТОВКА · 26.3",
               "playing": "СЕЙЧАС В ИГРЕ"}[state]
    sc.text(eyebrow, 64, 112, 10, "Medeo Mono", "Regular", s["muted"],
            ls="5%", height=16, valign="middle")
    sc.text("Minecraft 26.3", 64, 136, 72, "Medeo Sans", "Bold", s["fg"],
            width=600, lh="92%", ls="-1%")
    sc.text("Ваши миры, моды и серверы.\nБез аккаунта Microsoft и телеметрии.",
            64, 226, 14, "Medeo Sans", "Regular", s["muted"], width=520,
            lh="150%")

    bw, bh = 192, 52
    if state == "preparing":
        btn = sc.frame(64, 300, bw, bh, name="Primary button",
                       fill="#2A1710", radius=12, stroke=s["accent"], sw=1)
        sc.push(btn)
        sc.text("Подготовка 42%", 0, 0, 14, "Medeo Sans", "SemiBold", s["fg"],
                width=bw, height=bh, valign="middle", align="center")
        sc.rect(1, bh - 5, int((bw - 2) * 0.42), 3, fill=s["accent"], radius=2,
                parent=btn)
        sc.pop()
    elif state == "playing":
        btn = sc.frame(64, 300, bw, bh, name="Primary button",
                       fill="#1A1A1D", radius=12, stroke=s["accent"], sw=1)
        sc.push(btn)
        sc.rect(58, 21, 10, 10, fill=s["accent"], radius=2, parent=btn)
        sc.text("Остановить", 84, 0, 14, "Medeo Sans", "SemiBold", s["accent"],
                width=92, height=bh, valign="middle")
        sc.pop()
    else:
        btn = sc.frame(64, 300, bw, bh, name="Primary button",
                       fill=s["accent"], radius=12)
        sc.push(btn)
        sc.icon("play", 58, 20, 12, s["accent_fg"], filled=True)
        sc.text("Играть", 82, 0, 16, "Medeo Sans", "SemiBold", s["accent_fg"],
                width=90, height=bh, valign="middle")
        sc.pop()
    sc.text("Настройки", 320, 300, 13, "Medeo Sans", "Regular", s["muted"],
            width=82, height=bh, valign="middle")
    sc.icon("chevron", 412, 319, 14, s["muted"])

    if state == "preparing":
        sc.icon_text("download", 64, 388, 24, "Скачиваем 26.3", size=12,
                     color=s["accent"], text_size=13, text_color=s["fg"],
                     width=130)
        sc.text("42% · 1 024 МБ · 00:38", 220, 388, 10, "Medeo Mono",
                "Regular", s["muted"], height=24, valign="middle",
                features={"tnum": True})
    elif state == "playing":
        sc.icon_text("play", 64, 388, 24, "В игре · 12:04", size=12,
                     color=s["success"], text_size=13, text_color=s["fg"],
                     width=130)
        sc.text("26.3 · 60 FPS", 220, 388, 10, "Medeo Mono", "Regular",
                s["muted"], height=24, valign="middle",
                features={"tnum": True})
    else:
        sc.icon_text("check", 64, 388, 24, "Готово к запуску", size=12,
                     color=s["success"], text_size=13, text_color=s["fg"],
                     width=130)
        sc.text("26.3 · 1 024 МБ", 220, 388, 10, "Medeo Mono", "Regular",
                s["muted"], height=24, valign="middle",
                features={"tnum": True})

    sc.rect(64, 432, 640, 1, fill="#FFFFFF0A")
    sc.text("Ваши миры", 64, 456, 14, "Medeo Sans", "SemiBold", s["fg"],
            height=24, valign="middle")
    sc.text("4", 168, 456, 10, "Medeo Mono", "Regular", s["muted"], height=24,
            valign="middle")
    sc.text("Все миры", 624, 456, 11, "Medeo Sans", "Regular", s["muted"],
            width=80, height=24, valign="middle", align="right")

    rows = [
        ("New World", "Выживание · x 128 z -64", "2 ч назад", ["#3A2116", "#120C09"], "N", True),
        ("Build", "Творческий · 14 скриншотов", "3 дня назад", ["#1E2A33", "#0C1013"], "B", False),
        ("Hardcore Run", "Хардкор · без смерти", "на прошлой неделе", ["#2B1A1A", "#120A0A"], "H", False),
    ]
    for i, (name, meta, when, colors, letter, sel) in enumerate(rows):
        y = 484 + i * 64
        if sel:
            sc.rect(64, y, 640, 56, fill="#111113", radius=8)
            sc.rect(56, y + 8, 2, 40, fill=s["accent"], radius=1)
        sc.world_thumb(64, y + 8, 64, 48, colors, radius=8, letter=letter,
                       image=ART + "/" + ["thumb-new-world.png", "thumb-build.png", "thumb-hardcore.png"][i])
        sc.text(name, 144, y + 8, 15, "Medeo Sans", "SemiBold", s["fg"],
                height=24, valign="middle")
        sc.text(meta, 144, y + 30, 10, "Medeo Mono", "Regular", s["muted"],
                height=20, valign="middle")
        sc.icon_text("clock", 500, y + 8, 24, when, size=12,
                     color=s["muted"], text_size=10, family="Medeo Mono",
                     width=120, align="right", features={"tnum": True})
        sc.icon("chevron", 682, y + 22, 12, s["muted"])
        sc.rect(64, y + 62, 640, 1, fill="#FFFFFF07")

    # poster
    poster = sc.frame(744, 112, 472, 560, name="World poster", fill=s["surface"],
                      radius=20, stroke=s["border"], clip=True)
    sc.push(poster)
    sc.image(ART + "/hero-new-world.png", 0, 0, 472, 560, parent=poster,
             mode="fill")
    sc.rect(0, 0, 472, 560, fill="#0B0B0D4D", parent=poster)
    sc.rect(0, 0, 472, 132,
            gradient=grad(["#0B0B0DCC", "#0B0B0D00"]), parent=poster)
    sc.rect(24, 24, 132, 24, fill="#0B0B0D99", radius=6, parent=poster)
    sc.ellipse(34, 33, 6, 6, fill=s["accent"], parent=poster)
    sc.text("ВЫБРАННЫЙ МИР", 46, 24, 10, "Medeo Mono", "Regular", s["muted"],
            ls="6%", width=100, height=24, valign="middle", parent=poster)
    sc.rect(0, 300, 472, 260,
            gradient=grad(["#0B0B0D00", "#0B0B0DE6", "#0B0B0D"]), parent=poster)
    info = sc.frame(24, 408, 424, 128, fill="#0B0B0DCC", radius=16,
                    stroke="#FFFFFF14", sw=1, parent=poster)
    sc.push(info)
    sc.text("New World", 20, 16, 26, "Medeo Sans", "SemiBold", s["fg"],
            parent=info)
    sc.icon_text("clock", 20, 50, 24, "Выживание · 2 ч назад", size=12,
                 color=s["muted"], text_size=11, parent=info)
    sc.rect(20, 78, 116, 24, fill="#FFFFFF0E", radius=6, parent=info)
    sc.icon_text("map-pin", 28, 78, 24, "x 128 · z -64", size=12,
                 color=s["muted"], text_size=10, family="Medeo Mono",
                 width=84, parent=info, features={"tnum": True})
    sc.text("Открыть мир", 280, 78, 11, "Medeo Sans", "SemiBold", s["muted"],
            width=94, height=24, valign="middle", align="right", parent=info)
    sc.icon("arrow-up-right", 390, 84, 12, s["muted"], parent=info)
    sc.pop()
    sc.pop()

    # footer
    sc.rect(0, 688, W, 1, fill="#FFFFFF08", parent=root)
    sc.icon_text("keyboard", 64, 700, 16, "Ctrl+K  Команды", size=12,
                 color=s["muted"], text_size=10, family="Medeo Mono")
    sc.icon_text("help", 210, 700, 16, "F1  Помощь", size=12,
                 color=s["muted"], text_size=10, family="Medeo Mono")
    sc.text("Scoria 0.1.2", 1100, 700, 10, "Medeo Mono", "Regular", s["muted"],
            width=116, height=16, valign="middle", align="right")
    sc.pop()
    return root



# ================================================================ D states
def state_button(sc, x, y, state):
    s = D
    w, h = 192, 52
    if state == "focus":
        sc.rect(x - 4, y - 4, w + 8, h + 8, fill="none", radius=16,
                stroke=s["accent"], sw=2, opacity=0.55)
        state = "ready"
    if state in ("ready", "hover", "pressed"):
        fill = {"ready": s["accent"], "hover": "#FF7B52",
                "pressed": "#E0522A"}[state]
        f = sc.frame(x, y, w, h, fill=fill, radius=12)
        sc.push(f)
        sc.icon("play", 58, 20, 12, s["accent_fg"], filled=True)
        sc.text("Играть", 82, 0, 16, "Medeo Sans", "SemiBold", s["accent_fg"],
                width=90, height=h, valign="middle")
        sc.pop()
    elif state == "preparing":
        f = sc.frame(x, y, w, h, fill="#2A1710", radius=12,
                     stroke=s["accent"], sw=1)
        sc.push(f)
        sc.text("Подготовка 42%", 0, 0, 14, "Medeo Sans", "SemiBold", s["fg"],
                width=w, height=h, valign="middle", align="center")
        sc.rect(1, h - 5, int((w - 2) * 0.42), 3, fill=s["accent"], radius=2,
                parent=f)
        sc.pop()
    elif state == "playing":
        f = sc.frame(x, y, w, h, fill="#1A1A1D", radius=12,
                     stroke=s["accent"], sw=1)
        sc.push(f)
        sc.rect(50, 22, 8, 8, fill=s["accent"], radius=2, parent=f)
        sc.text("Остановить", 70, 0, 14, "Medeo Sans", "SemiBold",
                s["accent"], width=92, height=h, valign="middle")
        sc.pop()
    elif state == "download":
        f = sc.frame(x, y, w, h, fill=s["accent"], radius=12)
        sc.push(f)
        sc.icon("download", 44, 20, 12, s["accent_fg"])
        sc.text("Скачать 26.3", 64, 0, 13, "Medeo Sans", "SemiBold",
                s["accent_fg"], width=120, height=h, valign="middle")
        sc.pop()
    elif state == "error":
        f = sc.frame(x, y, w, h, fill="#1A0F0F", radius=12,
                     stroke="#E5484D", sw=1)
        sc.push(f)
        sc.icon("refresh", 50, 20, 12, "#E5484D")
        sc.text("Повторить", 70, 0, 13, "Medeo Sans", "SemiBold",
                "#E5484D", width=120, height=h, valign="middle")
        sc.pop()
    elif state == "disabled":
        f = sc.frame(x, y, w, h, fill=s["surface2"], radius=12,
                     stroke=s["border"], sw=1, opacity=0.55)
        sc.push(f)
        sc.text("Недоступно", 0, 0, 13, "Medeo Sans", "SemiBold",
                s["muted"], width=w, height=h, valign="middle",
                align="center", opacity=0.7)
        sc.pop()
    return f


def status_chip(sc, x, y, label, color, bg="#1A1A1D"):
    s = D
    sc.rect(x, y, 240, 32, fill=bg, radius=8)
    sc.ellipse(x + 12, y + 13, 6, 6, fill=color)
    sc.text(label, x + 26, y, 12, "Medeo Sans", "Medium", s["fg"],
            width=200, height=32, valign="middle")


def direction_states(sc, ox=0, oy=0):
    s = D
    root = sc.frame(ox, oy, W, H, key="states", name="D States",
                    fill=s["bg"], clip=True)
    sc.push(root)
    sc.rect(0, 0, W, 1, fill="#FFFFFF08")
    sc.text("D · Signal", 64, 56, 28, "Medeo Sans", "Bold", s["fg"],
            height=36, valign="middle")
    sc.text("состояния primary-кнопки и интерфейса", 64, 92, 13,
            "Medeo Sans", "Regular", s["muted"])

    states = [("ready", "Ready", "основное действие"),
              ("hover", "Hover", "accent_hover"),
              ("pressed", "Pressed", "accent_pressed"),
              ("focus", "Focus", "2 px ring · 4 px offset"),
              ("preparing", "Preparing 42%", "прогресс внутри кнопки"),
              ("playing", "Playing", "остановить игру"),
              ("download", "Download", "первый запуск"),
              ("error", "Error / Retry", "danger outline"),
              ("disabled", "Disabled", "нет подготовки")]
    for i, (state, label, note) in enumerate(states):
        col, row = i % 3, i // 3
        x = 64 + col * 256
        y = 160 + row * 120
        state_button(sc, x, y, state)
        sc.text(label, x, y + 60, 10, "Medeo Mono", "Regular", s["muted"])
        sc.text(note, x, y + 74, 10, "Medeo Sans", "Regular", s["muted"])

    sc.text("СТАТУСЫ", 860, 132, 9, "Medeo Mono", "Regular", s["muted"],
            ls="6%")
    status_chip(sc, 860, 156, "Готово к запуску", s["success"])
    status_chip(sc, 860, 196, "Подготовка 42%", s["accent"])
    status_chip(sc, 860, 236, "В игре · 12:04", "#7FE0D0")
    status_chip(sc, 860, 276, "Ошибка подготовки", "#E5484D", "#2B1A1A")

    sc.text("ПРОГРЕСС", 860, 332, 9, "Medeo Mono", "Regular", s["muted"],
            ls="6%")
    sc.rect(860, 356, 280, 6, fill="#26262A", radius=3)
    sc.rect(860, 356, 118, 6, fill=s["accent"], radius=3)
    sc.text("42% · 1 024 МБ · 00:38", 860, 370, 9, "Medeo Mono",
            "Regular", s["muted"])

    sc.text("ТОСТ", 860, 410, 9, "Medeo Mono", "Regular", s["muted"],
            ls="6%")
    sc.frame(860, 434, 280, 48, fill=s["surface2"], radius=8,
             stroke="#E5484D44", sw=1)
    sc.text("26.3 не скачался", 876, 434, 11, "Medeo Sans", "Medium",
            s["fg"], width=160, height=48, valign="middle")
    sc.text("Повторить", 1050, 434, 10, "Medeo Sans", "SemiBold",
            "#E5484D", width=74, height=48, valign="middle", align="right")

    sc.text("АНАТОМИЯ", 860, 510, 9, "Medeo Mono", "Regular", s["muted"],
            ls="6%")
    for i, line in enumerate(["192 × 52 · radius 12",
                              "play 12 · gap 12 · text 16",
                              "focus ring 2 px · 4 px offset",
                              "hover 120 ms · press 120 ms"]):
        sc.text(line, 860, 534 + i * 18, 10, "Medeo Mono", "Regular",
                s["muted"])

    sc.text("Scoria 0.1.2 · Signal", 64, 700, 9, "Medeo Mono",
            "Regular", s["muted"], height=16, valign="middle")
    sc.pop()
    return root


# ================================================================ E
E = dict(
    canvas="#F1F1F1", shell="#FFFFFF", surface="#FFFFFF", ink="#111214",
    muted="#6E7175", border="#E3E3E3", line="#ECECEC",
    accent="#111214", accent_fg="#FFFFFF", soft="#F2F2F2",
)


def direction_e(sc, ox=0, oy=0):
    s = E
    root = sc.frame(ox, oy, W, H, key="e", name="E White", fill=s["canvas"],
                    clip=True)
    sc.push(root)
    shell = sc.frame(16, 16, 1248, 688, name="Shell", fill=s["shell"],
                     radius=24, stroke=s["border"], clip=True)
    sc.push(shell)

    # top bar
    sc.text("Scoria", 48, 36, 15, "Onest", "SemiBold", s["ink"], height=32,
            valign="middle")
    sc.text("0.1.2", 104, 38, 10, "Medeo Mono", "Regular", s["muted"],
            height=28, valign="middle")
    nav = [("Играть", 528, 56, True), ("Моды", 608, 48, False),
           ("Серверы", 680, 72, False)]
    for label, x, w, on in nav:
        sc.text(label, x, 36, 13, "Onest", "SemiBold" if on else "Regular",
                s["ink"] if on else s["muted"], width=w, height=32,
                valign="middle")
    sc.rect(528, 68, 56, 2, fill=s["accent"], radius=1)
    sc.rect(1060, 40, 82, 24, fill=s["soft"], radius=8)
    sc.ellipse(1070, 49, 6, 6, fill=s["muted"])
    sc.text("Офлайн", 1082, 40, 10, "Onest", "Medium", s["ink"], width=52,
            height=24, valign="middle")
    sc.window_controls(45, s["muted"], x=1160)
    sc.rect(48, 76, 1184, 1, fill=s["line"])

    # hero
    hero = sc.frame(48, 100, 640, 320, name="Continue card",
                    gradient=grad(["#F4F4F4", "#FFFFFF", "#F7F7F7"], "linear",
                                  (0, 0), (1, 1)), radius=20, clip=True)
    sc.push(hero)
    imgf = sc.frame(320, 0, 320, 320, clip=True, parent=hero)
    sc.image(ART + "/hero-soft.png", 0, 0, 320, 320, parent=imgf, mode="fill")
    sc.rect(300, 0, 120, 320,
            gradient=grad(["#FFFFFF", "#FFFFFF00"], "linear", (0, 0), (1, 0)),
            parent=hero)
    sc.rect(320, 0, 1, 320, fill="#D8D8D8", parent=hero)
    sc.icon("clock", 32, 30, 12, s["muted"], parent=hero)
    sc.text("ПРОДОЛЖИТЬ", 52, 28, 10, "Medeo Mono", "Regular", s["muted"],
            ls="6%", height=16, valign="middle")
    sc.text("New World", 32, 52, 40, "Onest", "ExtraBold", s["ink"],
            width=460, lh="100%", ls="-0.5%")
    sc.icon("map-pin", 32, 106, 12, s["muted"], parent=hero)
    sc.text("Выживание · x 128 · z -64 · 2 ч назад", 52, 104, 12, "Onest",
            "Regular", s["muted"], width=260, height=16, valign="middle")
    btn = sc.frame(32, 216, 184, 48, key="e_play", name="Primary button",
                   fill=s["accent"], radius=14)
    sc.push(btn)
    sc.icon("play", 58, 18, 12, s["accent_fg"], filled=True)
    sc.text("Играть", 82, 0, 15, "Onest", "SemiBold", s["accent_fg"],
            width=76, height=48, valign="middle")
    sc.pop()
    sc.text("26.3 · 1 024 МБ", 32, 276, 10, "Medeo Mono", "Regular",
            s["muted"], height=20, valign="middle")
    sc.pop()

    # right cards
    c1 = sc.frame(712, 100, 520, 152, name="Status card", fill=s["surface"],
                  radius=18, stroke=s["line"])
    sc.push(c1)
    sc.text("СОСТОЯНИЕ", 24, 20, 10, "Medeo Mono", "Regular", s["muted"],
            ls="6%", height=16, valign="middle")
    sc.icon("check", 24, 45, 14, s["ink"])
    sc.text("Готово", 52, 36, 26, "Onest", "Bold", s["ink"], height=32,
            valign="middle")
    sc.icon("hard-drive", 24, 78, 12, s["muted"])
    sc.text("26.3 · 1 024 МБ", 44, 76, 10, "Medeo Mono", "Regular",
            s["muted"], height=16, valign="middle", features={"tnum": True})
    sc.rect(24, 110, 472, 6, fill="#EDEDED", radius=3)
    sc.rect(24, 110, 472, 6, fill=s["accent"], radius=3)
    sc.text("100%", 452, 122, 10, "Medeo Mono", "Regular", s["muted"],
            width=44, align="right", features={"tnum": True})
    sc.pop()

    c2 = sc.frame(712, 268, 520, 152, name="Stats card", fill=s["surface"],
                  radius=18, stroke=s["line"])
    sc.push(c2)
    stats = [("folder", "4", "мира"), ("package", "12", "модов"),
             ("server", "2", "сервера"), ("shield-off", "0", "телеметрии")]
    for i, (ic, n, label) in enumerate(stats):
        x = 24 + i * 118
        sc.icon(ic, x, 18, 14, s["muted"])
        sc.text(n, x, 34, 30, "Onest", "ExtraBold", s["ink"],
                features={"tnum": True})
        sc.text(label, x, 80, 11, "Onest", "Regular", s["muted"])
        if i < 3:
            sc.rect(x + 106, 22, 1, 66, fill=s["line"])
    sc.pop()

    # shelf
    sc.text("Ваши миры", 48, 448, 14, "Onest", "SemiBold", s["ink"],
            height=24, valign="middle")
    sc.text("4", 140, 448, 10, "Medeo Mono", "Regular", s["muted"],
            height=24, valign="middle")
    sc.text("Все миры", 1100, 448, 11, "Onest", "Regular", s["muted"],
            width=132, height=24, valign="middle", align="right")

    worlds = [
        ("New World", "Выживание", ["#3A2116", "#120C09"], "N"),
        ("Build", "Творческий", ["#1E2A33", "#0C1013"], "B"),
        ("Hardcore Run", "Хардкор", ["#2B1A1A", "#120A0A"], "H"),
    ]
    for i, (name, mode, colors, letter) in enumerate(worlds):
        x = 48 + i * 404
        c = sc.frame(x, 476, 376, 168, name=f"World {name}", fill=s["surface"],
                     radius=18, stroke=s["line"], clip=True)
        sc.push(c)
        sc.world_thumb(12, 12, 352, 92, colors, radius=12, letter=letter,
                       letter_color="#FFFFFF14", parent=c,
                       image=ART + "/" + ["thumb-new-world.png", "thumb-build.png",
                                          "thumb-hardcore.png"][i])
        sc.text(name, 16, 112, 15, "Onest", "SemiBold", s["ink"], width=270,
                parent=c)
        sc.text(mode, 16, 136, 10, "Medeo Mono", "Regular", s["muted"],
                parent=c)
        sc.text("Открыть", 276, 132, 10, "Onest", "SemiBold", s["ink"],
                width=70, height=20, valign="middle", align="right",
                parent=c)
        sc.icon("arrow-up-right", 356, 136, 12, s["ink"], parent=c)
        sc.pop()

    sc.text("Локальные миры · Modrinth", 48, 668, 10, "Medeo Mono",
            "Regular", s["muted"], height=16, valign="middle")
    sc.text("Scoria 0.1.2", 1100, 668, 10, "Medeo Mono", "Regular",
            s["muted"], width=132, height=16, valign="middle", align="right")
    sc.pop()
    sc.pop()
    return root


# ================================================================ build
def build(mode):
    sc = Scene()
    if mode in ("all", "d"):
        direction_d(sc)
    if mode in ("all", "preparing"):
        direction_d(sc, 1280 if mode == "all" else 0, 0,
                    state="preparing", name="D Preparing")
    if mode in ("all", "playing"):
        direction_d(sc, 2560 if mode == "all" else 0, 0,
                    state="playing", name="D Playing")
    if mode in ("all", "states"):
        direction_states(sc, 0, 800 if mode == "all" else 0)
    if mode in ("all", "e"):
        direction_e(sc, 1280 if mode == "all" else 0, 800 if mode == "all" else 0)
    return sc


if __name__ == "__main__":
    mode = sys.argv[1] if len(sys.argv) > 1 else "all"
    sc = build(mode)
    with open(f"{mode}.json", "w", encoding="utf-8") as f:
        json.dump(sc.cmds, f, ensure_ascii=False, indent=1)
    print(f"{mode}.json: {len(sc.cmds)} commands, {sc.next_id - 2} layers")
