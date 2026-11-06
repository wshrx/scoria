#!/usr/bin/env python3
"""Audit the .figcraft file through the FigCraft MCP server (stdio)."""
import json
import os
import subprocess
import sys

FILE = os.path.join(os.path.dirname(__file__), "scoria-directions.figcraft")


class MCP:
    def __init__(self):
        self.p = subprocess.Popen(
            ["figcraft-cli", "mcp"],
            stdin=subprocess.PIPE, stdout=subprocess.PIPE,
            stderr=subprocess.PIPE, text=True, bufsize=1,
        )
        self.next_id = 1

    def send(self, method, params=None, notify=False):
        msg = {"jsonrpc": "2.0", "method": method}
        if params is not None:
            msg["params"] = params
        if not notify:
            msg["id"] = self.next_id
            self.next_id += 1
        self.p.stdin.write(json.dumps(msg) + "\n")
        self.p.stdin.flush()
        return None if notify else msg["id"]

    def wait(self, rid):
        while True:
            line = self.p.stdout.readline()
            if not line:
                err = self.p.stderr.read()
                raise RuntimeError(f"MCP closed: {err}")
            line = line.strip()
            if not line:
                continue
            try:
                msg = json.loads(line)
            except json.JSONDecodeError:
                continue
            if msg.get("id") == rid:
                if "error" in msg:
                    raise RuntimeError(msg["error"])
                return msg["result"]

    def call(self, name, arguments):
        rid = self.send("tools/call", {"name": name, "arguments": arguments})
        result = self.wait(rid)
        texts = [c.get("text", "") for c in result.get("content", [])
                 if c.get("type") == "text"]
        return json.loads(texts[0]) if texts and texts[0].strip().startswith(("{", "[")) else texts

    def close(self):
        try:
            self.p.stdin.close()
        except Exception:
            pass
        self.p.terminate()


def walk(node, out, depth=0):
    out.append((depth, node))
    for child in node.get("children", []) or []:
        walk(child, out, depth + 1)


def main():
    m = MCP()
    rid = m.send("initialize", {
        "protocolVersion": "2025-06-18",
        "capabilities": {},
        "clientInfo": {"name": "scoria-audit", "version": "1.0"},
    })
    m.wait(rid)
    m.send("notifications/initialized", notify=True)

    rid = m.send("tools/list")
    tools = m.wait(rid)
    names = [t["name"] for t in tools.get("tools", [])]
    print("MCP tools:", ", ".join(names))

    print("open:", m.call("open_file", {"path": FILE}))
    doc = m.call("inspect_document", {
        "depth": 12, "allPages": True,
        "fields": ["name", "type", "x", "y", "width", "height"],
    })
    pages = doc.get("pages") if isinstance(doc, dict) else None
    if pages is None and isinstance(doc, dict) and "document" in doc:
        pages = doc["document"].get("pages")
    if not pages:
        print(json.dumps(doc, ensure_ascii=False)[:2000])
        return

    bad = []
    frames = []
    total = 0
    for page in pages:
        nodes = []
        walk(page, nodes)
        for depth, n in nodes:
            total += 1
            x, y = n.get("x"), n.get("y")
            w, h = n.get("width"), n.get("height")
            if depth == 1:
                frames.append((n.get("name"), x, y, w, h))
            if isinstance(x, (int, float)) and isinstance(y, (int, float)):
                if x % 4 or y % 4:
                    bad.append((n.get("type"), n.get("name"), x, y))

    print(f"nodes: {total}")
    print("top-level frames:")
    for f in frames:
        print("  ", f)
    print(f"nodes off the 4 px grid: {len(bad)}")
    for n in bad[:40]:
        print("  ", n)
    m.close()


if __name__ == "__main__":
    main()
