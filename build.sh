#!/usr/bin/env bash
set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
BOLD='\033[1m'
NC='\033[0m' # No Color

echo -e "\n${BOLD}🔍 Running Tests...${NC}"
cargo test --release -- --nocapture | tee /tmp/arc_test_output.log

# Show summary: Passed + Failed
echo -e "\n${BOLD}📋 Test Summary:${NC}"
grep -E "test result:|failures:|FAILED|ok" /tmp/arc_test_output.log || true

# Optional: Show failed test names only
FAILED=$(grep '^failures:' /tmp/arc_test_output.log | wc -l)
if [ "$FAILED" -gt 0 ]; then
    echo -e "\n${RED}❌ Some tests failed:${NC}"
    grep '^failures:' -A 5 /tmp/arc_test_output.log
    exit 1
else
    echo -e "\n${GREEN}✅ All tests passed!${NC}"
fi

echo -e "\n${BOLD}⚙️ Building Release Binary...${NC}"
cargo build --release --quiet

echo -e "\n${BOLD}📦 Copying to ~/bin...${NC}"
cp target/release/arc ~/bin/arc

echo -e "\n${GREEN}✅ Done. Binary is now at ~/bin/arc${NC}\n"
