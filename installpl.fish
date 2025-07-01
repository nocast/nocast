#!/usr/bin/fish

if test (count $argv) -ne 3
    echo "Error: Please provide exactly 3 arguments: name, path, target_path"
    exit 1
end

set name $argv[1]
set -l original_dir (pwd)
set dir $original_dir/$argv[2]
set target $original_dir/$argv[3]

echo "Checking if it is a plugin..."
# Validate the input is a directory
if not test -d "$dir/$name"
    echo "Error: '$dir/$name' is not a valid directory"
    exit 1
end

# Check for manifest.toml existence
if test -f "$dir/$name/manifest.toml"
    echo "Success"
else
    echo "manifest.toml not found in $dir/$name"
    exit 1
end

echo "Building plugin `$name` from dir `$dir`..."
cd "$dir/$name"
cargo build --release
echo "Build succeeded!"

echo "Validating target_path..."
if not test -d "$target"
    echo "Error: '$target' is not a valid directory"
    exit 1
end

echo "Installing plugin at `$target`..."
echo "	Copying manifest..."
cp $dir/$name/manifest.toml $target/$name.toml
echo "	Copying plugin core..."
cp $dir/$name/target/release/lib$name.so $target/$name.so

echo "Plugin installed!!"
