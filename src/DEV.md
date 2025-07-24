# Developing guide for nocast plugins
This is a short and incomplete guide for developing plugins that can be installed in the Nocast Launcher.
Nocast plugins are written in **Rust**.

## Creating the project
For easily creating the rust project that later will become the plugin, we've created a template that can be used with `cargo-generate`
with the following command:
```bash
cargo generate nocast/plugintemplate
```
This template will ask for plugin name (project name), author and license. The last one needs to be open-source.

## The manifest file
At the main directory of the crate, there will be a file called `manifest.toml`. This file describes what actions your plugin can do,
and also it's name, author and license.

### An `action`
An action has the following structure:
- `name`: the action's name. It doesn't need to be the name of the function it calls, but it is better that way for easier organization.
- `expression`: a regex expression which the user query will be matched agains to determine if that action will run (use `(.*)` if you need it to always run).
- `function`: the name of the function (inside the `lib.rs` file) that the action calls.
- `autorun`: either `true` or `false`. Decides if the action will run in the case that the regex expression matches the query.

An action is represented in the following form in the `manifest.toml`:
```toml
[[actions]]
name = "sample"
expression = "(.*)"
function = "sample"
autorun = true
```

Multiple actions can be represented using multiple times the `[[actions]]` keyword:
```toml
[[actions]]
name = "first"
expression = "(.*)"
function = "first"
autorun = true

[[actions]]
name = "second"
expression = "(.*)"
function = "second"
autorun = false
```

## The `lib.rs` file
This is the main file of your plugin. You can have other files that need to be included in it using `mod subfile`.
This file contains all the functions that your actions can call.

### Using `nocast/plugincore`
This module is essential for the creation of plugins. If you create the plugin using the template, the import statement will already be
at the top of your `lib.rs` file.

### A `function`
Functions need to be determined and exported in a specific way, and have a special input and output, as they are used from the main
program.

This is how to write a function:
```rust
use nocast_plugincore::*;

#[unsafe(no_mangle)]
pub extern "C" fn sample_function(input: plugin_input) -> plugin_output {
    let input_vec: Vec<String> = parse_input(input);

    let mut res: Vec<ActionOutput> = Vec::new();
    res.push(ActionOutput {name: "some".to_string(), description: "thing".to_string(), target: "else,a".to_string()});

    return prepare_output(res);
}

#### Input and output
```
You need to use the `parse_input` and `prepare_output` functions to convert the input to a usable value and the output to a value that
can be passed to the main program.

When parsed, the input is a `Vec<String>`, which is a list of strings that can be expaded dinamically.

Before converting, the output is a `Vec<ActionOutput>`, which is a list of `ActionOutput` objects, which have the following structure:
```rust
struct ActionOutput {
    pub name: String,
    pub description: String,
    pub target: String,
}
```
The name and description elements are very easy to understand. The target element is a string containing the function that will run
when the item is runned, followed by a context string that will be passed to that function. It has the following structure: `target_function,context`.

Multiple functions can be in the `lib.rs` file, as it's the only one that can contain them. 

All of them need to have the same structure.
