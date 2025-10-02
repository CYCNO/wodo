Hello Programmers :)

So If you are seeing this it means you want to fix some bugs, add some features or just reading to understand how wodo works, which is very appreciated and before we begin into coding part, let's understand how wodo even works.

So by default you will have a settings.json file (which automatically will be created after you run the program) and in that file it will look something like this:

`settings.json`
```json
[
  {
    "current_branch": "main",
    "available_branches": [
      "main"
    ]
  }
]
```
so it consist of two items/elements and that are our current branch that is by default is main and all the available branches, which user will create in future.

and when a user create a new branch using `wodo branch create NewBranch` it update available_branch with new item of `NewBranch` and a new file of NewBranch.json got created and when user add new todo, the program first see what is the current branch and add todos in the branch json file making it look like this:

`NewBranch.json`
```json
[
  {
    "message": "hello",
    "done": true
  }
]
```

and adding new todo will expand this file.

and here is the file path of all the created files/branches:

- Windows: `C:\Users\<Username>\AppData\Roaming\wodo`
- MacOs: `~/Library/Application Support/wodo`
- Linux: `~/.config/wodo`

and when user switch to other branch it just set the current_branch to the branch user want to switch and now all the todo go to the current_branch(i.e the switched branch), for more technical part you can see the code.

Now let's talk about coding part

First install and run wodo:
```
git clone https://github.com/ERRORLY/wodo # Or download the code
cd wodo
cargo run help
```
this should act as `wodo help` and so for other args.

For now we are using `serde_json`, `serde`, `dirs` as dependencies for manipulating the json and files, you can see in `Cargo.toml` if we added more in future

There is currently 3 files in src and that are:
`actions.rs` - all the todos actions are done here.
`branch.rs`  - all the branch actions are done here.
`main.rs`    - all the command logic are here on how command will work.

So if you are thinking to contribute, first read our codebase(it's not that big for now), and than you can see how we manage our code, for each command we like to make a separate functions in their specific files, for e.g. if we create a new branch command we will first add the command name in it's enums and it's logic in `main.rs` and its functionality in `branch.rs` and will connect it's enums to the functionality (see the codebase for more understanding), and in the last we will update `help` command which you will find in `actions.rs`.

> Side Note: I am still learning the rust language, so there can be some programming mistakes which i have made, so you can either ignore it or let me know.
