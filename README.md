# Timetable

A simple to use but deep command line tool for viewing and editing timetables made in Rust.

## Running

Lists the whole timetable if ran without arguments

```sh
timetable 
Run without arguments to simply print the timetable

USAGE:
    timetable [OPTIONS]

OPTIONS:
    -a, --add-period <ADD_PERIOD>    Add a period, uses the format `-a [class name],[day],[period]`
        --add-class <ADD_CLASS>      Add a class
    -c, --config <CONFIG>            Use a different configuration path (defaults to ~/.timetable)
    -h, --help                       Print help information
    -r, --rm-period <RM_PERIOD>      Remove a period from the timetable, uses the format `-r
                                     [day],[period]`
    -t, --timetable                  List the full timetable
```
