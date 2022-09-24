# Canvas grading

Easy way of downloading student submissions for grading in Canvas

## How to use

Compile with:

```bash
cargo build --release
```

Move the binary into path somewhere. For example:

```bash
cp target/release/grader $HOME/.local/bin/
```

Then you need to place this into your `.zshrc`, `.bashrc` etc... if it's not already in path:

```bash
export PATH=$PATH:$HOME/.local/bin
```

Now you can move into where you want to install the submissions

Before you run the program you need to create a `.env` file with a token, course id and assignment id

```bash
TOKEN="xxx"
COURSE=12345
ASSIGNMENT=12345
```

You can also override all of the variables with command line arguments. See `-h or --help`

```bash
USAGE:
grader --token <TOKEN> --course <COURSE> --assignment <ASSIGNMENT>

OPTIONS:
-a, --assignment <ASSIGNMENT>    [env: ASSIGNMENT=12345]
-c, --course <COURSE>            [env: COURSE=12345]
-h, --help                       Print help information
-t, --token <TOKEN>              [env: TOKEN=xxx]
-V, --version                    Print version information

```

Now you can run the binary `grader` to download submissions for the students in your section.

This should install all submissions for the specified course and assignment that you need to grade.
The script will automatically try to find the section(s) that you are a TA for, and will only download submissions for those
