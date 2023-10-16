#!/bin/bash

# Change folder to release
echo "Changing folder"
cd ../../release/
echo "Check if the executable file exists"
if [ ! -f "$1" ]; then
  echo "Error: The executable file '$1' does not exist."
  exit 1
fi

echo "Check if the executable file is already in the PATH"
if [[ ":$PATH:" == *":$1:" ]]; then
  echo "The executable file '$1' is already in the PATH."
  exit 0
fi

echo "Add the executable file to the PATH"
PATH="$1:$PATH"

echo "Export the PATH variable"
export PATH

# Print the PATH variable
echo "The PATH variable is now: $PATH"
