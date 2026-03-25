# Qblock
Make blocks quicker.

# Install
Note: The bash script is kinda bæsj sooo uhh yeah

You can build it yourself using one of the `build.*` scripts or download a release which has the binary premade. If you are not om M series MacOs you will need to change the `Cargo.toml` target.

Run the install script which will create the following files in `~/.config/qblock/`

- template
- template.php
- block.json
- assets
  - template.scss
uninstall.sh
uninstall.fish
qblock

it will then create a symlink from `qblock` to `/usr/bin/qblock@`

# Usage
`qblock <blockname>` And as long as you're in a wordpress directory it will create the block in `wp-content/themes/site_name/src/blocks` it does not work on older sites which do not have this structure in place
