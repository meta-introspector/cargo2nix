#!/bin/bash

FILE_PATH="submodules/rust/compiler/expand_utils_macros/src/ast_fragments.rs"

echo "Applying sed changes to $FILE_PATH"

# Fix for @mut_visit_with
# Match the line containing __generate_mut_visit_with_many_arm! with the problematic argument.
# Replace `(\$(\$args)*)` with `\$(\$args)*`
sed -i -E 's/(__generate_mut_visit_with_many_arm! \{[^,]+,[^,]+,[^,]+,[^,]+,[^,]+,)\(\$args\)(.*)/\1\$args\2/' "$FILE_PATH"

# Fix for @add_placeholders
sed -i -E 's/(__generate_add_placeholders_arm! \{[^,]+,[^,]+,[^,]+,[^,]+,[^,]+,)\(\$args\)(.*)/\1\$args\2/' "$FILE_PATH"

# Fix for @visit_with
sed -i -E 's/(__generate_visit_with_many_arm! \{[^,]+,[^,]+,[^,]+,[^,]+,[^,]+,)\(\$args\)(.*)/\1\$args\2/' "$FILE_PATH"

# Fix for @to_string
sed -i -E 's/(__generate_to_string_many_arm! \{[^,]+,[^,]+,[^,]+,[^,]+,[^,]+,)\(\$args\)(.*)/\1\$args\2/' "$FILE_PATH"

echo "Changes applied. Please review the file and rebuild."
