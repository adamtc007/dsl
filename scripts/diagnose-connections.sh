#!/bin/bash

echo "=== Antigravity & Google Gemini API Diagnostic ==="
echo ""

# 1. Check Env Vars
echo "1. Checking environment variables..."
if [ -n "$GEMINI_API_KEY" ]; then
    echo "  [OK] GEMINI_API_KEY is set (starts with: ${GEMINI_API_KEY:0:7}...)"
else
    echo "  [FAIL] GEMINI_API_KEY is not set."
fi

if [ -n "$ANTIGRAVITY_API_KEY" ]; then
    echo "  [OK] ANTIGRAVITY_API_KEY is set (starts with: ${ANTIGRAVITY_API_KEY:0:7}...)"
else
    echo "  [FAIL] ANTIGRAVITY_API_KEY is not set."
fi

# 2. Check Keychain
echo ""
echo "2. Checking macOS Keychain status for GEMINI_API_KEY..."
KEYCHAIN_KEY=$(security find-generic-password -a "$USER" -s "GEMINI_API_KEY" -w 2>/dev/null)
if [ -n "$KEYCHAIN_KEY" ]; then
    echo "  [OK] GEMINI_API_KEY found in Keychain (starts with: ${KEYCHAIN_KEY:0:7}...)"
else
    echo "  [WARNING] GEMINI_API_KEY not found in Keychain."
fi

# 3. Check Google Cloud ADC (OAuth)
echo ""
echo "3. Checking for conflicting OAuth / Google Application Default Credentials..."
if [ -f "$HOME/.config/gcloud/application_default_credentials.json" ]; then
    echo "  [INFO] Found application_default_credentials.json (OAuth/Service Account could be active)."
else
    echo "  [OK] No application_default_credentials.json file found (defaulting to API key)."
fi

# 4. Test API Connectivity
echo ""
echo "4. Testing connectivity to Google Gemini API..."
RESPONSE_CODE=$(curl -s -o /dev/null -w "%{http_code}" -H "Content-Type: application/json" -d '{"contents":[{"parts":[{"text":"Hello"}]}]}' "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key=$GEMINI_API_KEY")

if [ "$RESPONSE_CODE" -eq 404 ] || [ "$RESPONSE_CODE" -eq 200 ]; then
    echo "  [OK] Successfully authenticated to Google Gemini API (HTTP $RESPONSE_CODE)."
elif [ "$RESPONSE_CODE" -eq 400 ]; then
    echo "  [FAIL] Authentication failed: Google API rejected the key (HTTP $RESPONSE_CODE - API key not valid)."
else
    echo "  [FAIL] Connection failed with HTTP code $RESPONSE_CODE."
fi
