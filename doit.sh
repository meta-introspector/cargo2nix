
nix develop ../../external/gemini-cli/ -c bash -c "
  ../../external/gemini-cli/bundle/gemini.js --output-format json \
			      --model gemini-2.5-flash
"
