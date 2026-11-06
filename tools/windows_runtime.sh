#!/bin/sh
# Copies VCRUNTIME140.dll beside a Windows build, but only when a binary there
# imports it: the Rust exes are built with +crt-static and the GPUIX addon can be
# built from source the same way, so usually nothing is needed.
set -eu
destination=${1:?usage: windows_runtime.sh <folder>}
dll=vcruntime140.dll

needed=0
for file in "$destination"/*.exe "$destination"/*.node; do
  [ -f "$file" ] || continue
  if grep -aq "VCRUNTIME140.dll" "$file"; then
    echo "$(basename "$file") imports $dll"
    needed=1
  fi
done

if [ "$needed" -eq 0 ]; then
  echo "No binary imports $dll; nothing to copy."
  exit 0
fi

if [ -f "$destination/$dll" ]; then
  exit 0
fi

vswhere="/c/Program Files (x86)/Microsoft Visual Studio/Installer/vswhere.exe"
if [ -x "$vswhere" ]; then
  install=$("$vswhere" -latest -products '*' -property installationPath 2>/dev/null | tr -d '\r' || true)
  if [ -n "$install" ] && [ -d "$install/VC/Redist/MSVC" ]; then
    found=$(find "$install/VC/Redist/MSVC" -path "*/x64/Microsoft.VC143.CRT/$dll" 2>/dev/null | sort | tail -n 1)
    if [ -n "$found" ]; then
      cp "$found" "$destination/$dll"
      echo "Copied $dll from $found"
      exit 0
    fi
  fi
fi

echo "warning: $dll not found; the build needs the Microsoft Visual C++ Redistributable x64" >&2
