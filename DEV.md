# Developing plugins for NoCast
This guide aims to make it eazy to create plugins for the NoCast app.

## Manifest
When creating a plugin, you need to create a file called `manifest.toml`, where you describe all the plugin can do, the name, author
license...

This is the required manifest structure:
```toml
name = "plugin_name"
version = "0.1.0"
author = "your_name"
license = "MIT" # Needs to be an Open-Source license
source = "https://github.com/your_name/plugin_name" # The repo or hosting of the plugin's source code

# Description of the actions...
```
This is only the header of the manifest. Then, you need to describe all the actions your plugin can do:
```toml
# Manifest header...

# Action 1
[[actions]]
name = "Some action" # The action's name
expression = "(.*)" # The regex expression to match and input into your action
file = "some-action.js" # The action's script'
autorun = true # If the action should run autommatically or needs to be started by the user

# Action 2
[[actions]]
name = "Another action"
expression = "([a-z]*)"
file = "another-.js"
autorun = flase
````
These parameters need to be explained one by one.
