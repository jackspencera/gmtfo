# GMTFO (Get Me The F*** Out)

Simple tool built in rust to search up your path list for a directory that contains one of the configured target folders. Main use case envisioned in development is for when you are working in a big repo with nested files and you wanna jump back to the root directory that contains a `.git` folder.

## Why this instead of using the builtin git functions
For git the same functionality can be achieved via simple commands like `git rev-parse --show-toplevel`. This tool was built to be more general purpose and configurable but also and primarily because why not

## Installation
Build the binary and move it into your path (how very trusting of you).
Then you simply need to create a shell function for invoking the cd operation with the tool as programs aren't able to control the terminals working dir.

For zsh the function looks like
```sh
b() {
  DIR=$(gmtfo);
  cd $DIR;
}
```

## Configuration
The tool enables you to add target folders that you wish to be able to jump to in the `.gmtfo` config file (by default will be at `~/.config/.gmtfo`. By default will be configured for `.git` and `.config`

Additionally the tool can be configured to optionally include the current directory so if you are in the highest level git folder, you wont accidentally cd out of it using the tool (but you can if you want)

## Future work
Add configuration options via the cli rather than requiring the toml to be modified directly

More features to come?
