# A simple timer

A little tool to execute programs after some time. Written in Rust.

Usage

```sh
timer [OPTIONS] --time <TIME> --command <COMMAND>
```

Options

```sh
-t, --time <TIME>
          The time before the task is executed

-c, --command <COMMAND>
          The command to execute

-f, --format <FORMAT>
          Time format

          [default: %H:%M:%S]

-d, --display <DISPLAY>
          Display remaining time ?

          [default: True]

-h, --help
          Print help information (use `-h` for a summary)
```

Example:

```sh
timer -t "11:45:14" -c "ls -l" -f "%H:%M:%S" -d "False"
```

