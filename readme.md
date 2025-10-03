<div align="center">
  <img src="/images/wodo-logo.png" alt="Wodo Logo"/>
  <strong><i>A CLI Todo Manager With Branches
</i></strong>
  <br>
</div>
<hr/>

A Simple yet Powerful Tool to manage your todos and organise it for different use cases through branches, Built in rust for fast performance

```
~$ wodo help
  Wodo CLI - Commands:
    add <message>     ➜ Add a new todo item
    show              ➜ Show all todos
    done <number>     ➜ Mark todo item as done
    delete <number>   ➜ Delete a todo item
    help              ➜ Show this help message
                ---
  Branch Related Commands:
    branch create <name>      ➜ Add a new branch
    branch show               ➜ Show all the branches
    branch switch <number>    ➜ Switch to other branches
    branch delete <number>    ➜ Delete a branch
```
```
~$ wodo show
      ------------------------------------------------------
     | No | Message (Showcase)                         | Box |
      ------------------------------------------------------
     | 1  | showcase wodo                              | [ ] |
     | 2  | hello people                               | [ ] |
      ------------------------------------------------------

```
Instead of creating `todo.txt` in your project folder, just run `wodo branch add ProjectName` :)

## ✨ Features
- **Fast** and lightweight, Build in rust.
- **Branch Based system** to organise todos
- **OpenSource**


## ✨ Installation

> **Checkout [release](https://github.com/CYCNO/wodo/releases/latest), for more info on which zip to download according to your os and architecture.**

### For Windows
1. Download the [Windows zip](https://github.com/CYCNO/wodo/releases/latest/download/wodo-windows.zip) and extract it.
2. Open **Command Prompt** or **PowerShell** in the same folder where you downloaded the file.
3. Run:
   ```powershell
   mkdir C:\Users\<YourName>\.wodo
   move wodo.exe C:\Users\<YourName>\.wodo\
   ```
4. Now test with:

   ```powershell
   wodo help
   ```
> Add the path to your PATH environment variable if not already included

### For Linux

1. Download the [Linux zip](https://github.com/CYCNO/wodo/releases/latest/download/wodo-x86_64-unknown-linux-gnu.zip) and extract it.
2. Open terminal in the download folder and run:
   ```bash
   chmod +x wodo
   sudo mv wodo /usr/local/bin/
   ```
3. Test the installation:
   ```bash
   wodo help
   ```

### For macOS

1. Download the [macOS zip](https://github.com/CYCNO/wodo/releases/latest/download/wodo-x86_64-apple-darwin.zip) and extract it.
2. Open terminal in the download folder and run:

   ```bash
   chmod +x wodo
   sudo mv wodo /usr/local/bin/
   ```
3. Test the installation:

   ```bash
   wodo help
   ```

### From Source Code
  - make sure to **[install rust](https://rust-lang.org/tools/install/) first** and than
  ```
  git clone https://github.com/CYCNO/wodo # Or download the code
  cd wodo
  cargo build --release
  ```
  and then you will find wodo executables (as per your OS) in **target/release/**

## ✨ Contribution or Support
If you want to **Contribute** or understand how it wodo works, see the [countribution.md](https://github.com/CYCNO/wodo/blob/main/contribution.md) Or if you want to add **features or report bugs**, create a new [issue](https://github.com/CYCNO/wodo/issues) or a [pull](https://github.com/CYCNO/wodo/pulls) request.
