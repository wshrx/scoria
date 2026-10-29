#!/bin/sh
# Copies the Visual C++ runtime DLLs the Rust exes and the GPUIX addon import
# (VCRUNTIME140.dll) beside a Windows build. They are not part of Windows itself.
set -eu
destination=${1:?usage: windows_runtime.sh <folder>}
dll=vcruntime140.dll

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
