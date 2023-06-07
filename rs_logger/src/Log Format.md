# Log Format

## Formats

### Log Entry Format:

```
[<timestamp> <log_type> <log_id>] <log_contents>
```

- `timestamp` - The time that the log was created
  - The time is recorded in UTC time
- `log_type` - The type of log
  - Available types are:
    - `INFO` - An information log
    - `WARN` - A warning log
    - `ERROR` - An error log
- `log_id` - The Log ID of the corresponding log
  - Used to obtain further details about a log

### Timestamp Format:

```
<HH:MM:SS>
```

- `HH` - Two digits representing the hour in a 24 hour format
- `MM` - Two digits representing the _Minutes_
- `SS` - Two digits representing the _Seconds_

### Log ID Format:

```
[crate][module][log_type][log_index]
[  F  ][  FF  ][   F    ][   FF    ]
```

- `crate` - One hexadecimal character representing the crate index
- `module` - Two hexadecimal characters representing the module path index
- `log_type` - One hexadecimal character representing the type of log
  - `1` - An _INFO_ log
  - `2` - A _WARN_ log
  - `3` - An _ERROR_ log
- `log_index` - Two hexadecimal characters representing the log index
- `log_contents` - The contents of the log
  - Can be multiline content

## Log File Example:

```
[16:23:48 INFO  10A102] Lorem ipsum
[16:23:48 WARN  D1F20F] Lorem ipsum
[16:23:48 ERROR 6083AF] Lorem ipsum
```
