#!/bin/bash

# Test script to query PBS Kids API and extract episode info
# Usage: ./test_pbs_api.sh [show-slug]

SHOW_SLUG="${1:-daniel-tigers-neighborhood}"
URL="https://producerplayer.services.pbskids.org/show-list/?shows=${SHOW_SLUG}&page=1&page_size=20&available=public&sort=encored_on&type=episode"

echo "Fetching episodes for: $SHOW_SLUG"
echo "URL: $URL"
echo ""
echo "Title | NOLA Code"
echo "------|----------"

curl -s "$URL" \
  -H "User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0" \
  -H "Accept: application/json" \
  | jq -r '.items[] | "\(.title) | \(.nola_episode)"' 2>/dev/null

if [ $? -ne 0 ]; then
  echo "Error: Failed to fetch or parse API response"
  echo "Raw response:"
  curl -s "$URL" \
    -H "User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0" \
    -H "Accept: application/json"
fi
