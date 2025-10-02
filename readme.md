<div style="display:flex; align-items: center; justify-content: center; gap: 30px;">
    <img src="/images/wodo-logo.png" alt="Wodo Logo"/>
</div>

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

## ✨ Features
- **Fast** and lightweight, Build in rust.
- **Branch Based system** to organise todos
- **OpenSource**

## ✨ Installation
- **For Windows**
  ```
  $dir = "$env:USERPROFILE\.wodo"
  mkdir $dir -Force | Out-Null
  Invoke-WebRequest -Uri "https://github.com/ERRORLY/wodo/releases/latest/download/wodo-windows.zip" -OutFile "$dir\wodo.zip"
  Expand-Archive "$dir\wodo.zip" -DestinationPath $dir -Force
  del "$dir\wodo.zip"
  setx PATH "$env:Path;$dir"

  ```
- **For MacOS**
  - This will work in most MacOS version.
  ```
  curl -L -o wodo.zip "https://github.com/ERRORLY/wodo/releases/latest/download/wodo-x86_64-apple-darwin.zip" && \
  unzip wodo.zip && \
  chmod +x wodo && \
  sudo mv wodo /usr/local/bin/ && \
  rm wodo.zip
  ```
  - If it doesn't work for your MacOS architecture you can download [other MacOS version](https://github.com/ERRORLY/wodo/releases/latest).
- **For Linux**
  ```
  curl -L -o wodo.zip "https://github.com/ERRORLY/wodo/releases/latest/download/wodo-x86_64-unknown-linux-gnu.zip" && \
  unzip wodo.zip && \
  chmod +x wodo && \
  sudo mv wodo /usr/local/bin/ && \
  rm wodo.zip
  ```
  - If your linux is musl architecture based than [download](https://github.com/ERRORLY/wodo/releases/latest) that one.

- **From Source Code**
  - make sure to **[install rust](https://rust-lang.org/tools/install/) first** and than
  ```
  git clone https://github.com/ERRORLY/wodo # Or download the code
  cd wodo
  cargo build --release
  ```
  and then you will find wodo executables (as per your OS) in **target/release/**

## ✨ Contribution or Support
If you want to **Contribute** or understand how it wodo works, see our [countribution.md](https://github.com/ERRORLY/wodo/blob/main/contribution.md) Or if you want to add **features or report bugs**, create a new [issue](https://github.com/ERRORLY/wodo/issues) in our github repo, or [contact us](/community)
