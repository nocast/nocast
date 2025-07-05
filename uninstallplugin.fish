#!/usr/bin/fish

set name $argv[1]
set target $argv[2]
set -l original_dir (pwd)

echo "- Checking if it is installed..."
if grep -q $name $original_dir/nocast.toml
	echo "	It is installed, uninstalling..."
else
	echo "Plugin is not installed"
    exit 1
end

echo "- Removing plugin at `$target`..."
echo "	- Removing manifest..."
rm -rf $target/$name.toml
echo "	- Removing plugin core..."
rm -rf $target/$name.so

echo "- Removing plugin from config file..."
grep -v ^$name "$original_dir/nocast.toml" > tmpfile && mv tmpfile "$original_dir/nocast.toml"

echo "Plugin removed"
