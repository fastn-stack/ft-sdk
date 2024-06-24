import os
import fnmatch

# Function to prepend a line to a file
def prepend_line_to_file(file_path, line):
    with open(file_path, 'r+') as file:
        content = file.read()
        file.seek(0, 0)
        file.write(line.lstrip('\r\n') + '\n' + content)

# Function to traverse directories and process .rs files
def add_comment_to_rs_files(root_dir, comment):
    for root, dirs, files in os.walk(root_dir):
        for filename in fnmatch.filter(files, '*.rs'):
            file_path = os.path.join(root, filename)
            prepend_line_to_file(file_path, comment)

# Directory containing the .rs files
directory_path = 'ft-stripe/src/async_stripe'

# Comment to be added
comment_line = '''// This code is taken from [async-stripe](https://github.com/arlyon/async-stripe/tree/0a00d31894191ee0c6b4bda31e0d52d59e8e93b7)
// Author: Alexander Lyon
// License under either of:
//      - Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
//      - MIT License (LICENSE-MIT or https://opensource.org/licenses/MIT)
'''

# Add the comment to all .rs files in the directory and subdirectories
add_comment_to_rs_files(directory_path, comment_line)

print("Comment added to all .rs files successfully.")
