# Canvas grading

Easy way of downloading student submissions for grading in Canvas

## How to use

Create a `.env` file with a token, course id and assignment id

```bash
TOKEN="xxx"
COURSE=12345
ASSIGNMENT=12345
```

Compile with:

```bash
cargo build --release
```

Move the binary into path somewhere. For example:

```bash
cp target/release/grader ~/.local/bin/
```

Now you can move into where you want to install the submissions and run: `grader`.

This should install all submissions for the specified course and assignment that you need to grade.
The script will automatically try to find the section(s) that you are a TA for, and will only download submissions for those
