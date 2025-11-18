cat todo.txt  | grep Patch  | cut -d/ -f2- | cut "-d)" -f1 | cut -d/ -f11 > ignore.txt
for x in `cat ignore.txt`; do sed -e's!$x =!#\$x =!g' .cargo/config.toml; done
