# Rollit

A small utility program that can "roll" based on named text files
Essentially the program uses the first command line argument to look up a text file inside the specified folder, picks a random line out of the file, and outputs it to stdout.

## Usage

Create a folder in your home directory at .local/share/rollit and fill it with named text files.
An example file would be d6, filled with the numbers 1 - 6, each on a new line. Another example is a file yn with one line for yes, and another for no.
This can be used for rolling on whatever list of lines you want however, so be creative!
Note that folder paths are followed. If you create a .local/share/rollit/[folder]/[somefile] you can roll on that file using rollit [folder]/[somefile].

### Command line arguments

Support for command line arguments is very slowly trickling into the Rollit implementation. A basic setup has been written, which should be fairly easy to extend with additional behaviour. 

#### Multiple rolls
The --count X or --repeats X argument can be supplied to rollit before the file is given. This will cause said file to be rolled on X amount of times.

`rollit --count [number] file`

## Building and deploying

Clone this repo and read the source code, it's short.
If you want, you can change the location in which the program looks for text files by alterting the .push(".local/share/rollit") to your desired location. Note that this path is relative to the home folder, though this can also be changed iwth a little more work should you so desire.
Run cargo build with the -r flag, you'll find an executable in the target/release folder. Copy this someplace that is accessed by your $PATH.

## Limitations

The current version will panic if the file does not exist, this is expected and okay, but not pretty.
There are no checks for ../ in the parsing of the file location! This would theoretically enable any file with sufficient permission settings to be accessed by the rollit utility. For now until an appropriate check is implemented, be mindful that this can be done. 