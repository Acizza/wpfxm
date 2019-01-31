wpfxm
=====

The goal of this program is to make it easier to manage Wine prefixes and their applications by streamlining common operations, saving state per prefix (like environment variables), and quickly being able to launch applications within a prefix.

Please note that the program's design is still unstable, so the following sections will be somewhat light in details. Run the program with `--help` to see all available commands, and with the command name and `--help` to see all of its flags.

Initial Configuration
=====================

To make things easier, the program creates and looks for all Wine prefixes inside of a set directory. By default, this directory is `/home/<USERNAME>/wine`. To change this, run the program like so: `wpfxm cfg set baseDir <ABSOLUTE PATH>`

Create A Prefix
===============

With the `new` command, you can create a new Wine prefix and set its state for applications.

For example, the following command will create a new prefix named `test`, run an application installer in the prefix, and launch all future applications in the prefix with the `LANG` environment variable set:
`wpfxm new test -r wine64 ~/Downloads/installer.exe -e LANG=ja_JP.utf8`

Detect Prefix Applications
==========================

Applications in a prefix must be detected and given a name before they can be launched. To do that, the `add` command can be used. It will scan all user data folders in the specified prefix and prompt you to select which .exe is the application you're looking for.

For example, the following command will prompt you to select the .exe that you want to associate with the name `game` within the `test` prefix:
`wpfxm add test game`

Note that you can use the `-a` flag when using the `new` command with the `-r` flag to skip having to run this command separately.

Running An Application
======================

With the `run` command, you can quickly launch any application added with the `add` command for a prefix. This command will launch the application with the state variables set for the prefix (ex. environment variables).

For example, the following command will run the application saved as `game` in the `test` prefix:
`wpfxm run test game`

If there is only one application saved for a prefix, you can omit the name of the application when using the `run` command.

Running Other Programs In A Prefix
==================================

When using the program to manage prefixes for games, you *(very)* often have to run tools like winetricks to make them work. To run arbitrary Linux programs that are "attached" to a specified prefix, you can use the `exec` command.

For example, the following will use winetricks to set the `test` prefix's Windows version to Windows XP:
`wpfxm exec test winetricks winxp`

Hooks
=====

To avoid performing repetitive commands on some or all prefixes, you can use hooks (which are simply shell scripts) to perform those tasks automatically.

An example would be to automatically fix Unreal Engine 4 games so they can launch: https://gitlab.com/Acizza/dotfiles/blob/desktop/.config/wpfxm/hooks/ue4_fix.sh

New hooks can be placed in `/home/<USERNAME>/.config/wpfxm/hooks/`. Keep in mind that you must mark them as executable with `chmod +x <FILE>`.

You can register a hook as a "setup" hook that runs when new prefixes are created via the [cfg](#Global-And-Prefix-Configuration) command, or run them with the `hook run` command.

With the `hook run` command, the specified hooks will run in all prefixes by default, but you can restrict them to run in only one prefix with the `-p` flag.

For example, the following command will run a hook named `setup_dxvk` in all registered prefixes:
`wpfxm hook run setup_dxvk`

Removing A Prefix
=================

With the `rm` command, you can quickly remove a prefix from disk and unregister it from the program.

You can use the `-d` flag to only unregister the prefix from the program, but keep the prefix data intact.

Likewise, you can use the `-p` flag to only remove the prefix data, but keep the prefix registered with the program.

Global And Prefix Configuration
===============================

The `cfg` command lets you edit the global and prefix configuration without having to hunt down specific files. Since this command is big, the best way to view its options is to view its help information.

Make sure to view the help on a specific setting to see all of its flags. Most allow you to append the existing setting, instead of overwriting them.