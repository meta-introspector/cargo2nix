cargo build > build.log 2>&1

grep error build.log | sort | uniq -c | sort -rn | head
