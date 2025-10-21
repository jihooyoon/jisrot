#!/usr/bin/env bash
set -e

# ==============================
# Self-Signed Code Signing Script for Ji Hoo Yoon
# Cross-platform (Linux/macOS)
# ==============================

NAME="Ji Hoo Yoon"
EXE="target/release/jisrot.exe"
DAYS=3650

if [ ! -f "$EXE" ]; then
  echo "❌ ERROR: File not found: $EXE"
  exit 1
fi

echo "=== Generating key and certificate ==="
openssl req -newkey rsa:4096 -nodes -keyout jihooyoon.key -x509 -days $DAYS -sha256 -subj "/CN=$NAME" -out jihooyoon.crt

echo "=== Creating PFX ==="
read -sp "Enter password for PFX: " PASS
echo
openssl pkcs12 -export -out jihooyoon.pfx -inkey jihooyoon.key -in jihooyoon.crt -passout pass:"$PASS"

echo "=== Signing $EXE ==="
osslsigncode sign -pkcs12 jihooyoon.pfx -pass "$PASS" \
  -n "jisrot.exe" \
  -i "https://github.com/jihooyoon/jisrot" \
  -t http://timestamp.digicert.com \
  -in "$EXE" -out "${EXE%.exe}-signed.exe"

echo "=== Verifying signature ==="
osslsigncode verify -in "${EXE%.exe}-signed.exe"

echo "✅ Done!"
echo "Generated files:"
echo " - jihooyoon.key"
echo " - jihooyoon.crt"
echo " - jihooyoon.pfx"
echo "Signed binary: ${EXE%.exe}-signed.exe"
echo
echo "To trust this cert on Windows:"
echo "  -> import jihooyoon.crt into Trusted Root Certification Authorities"
