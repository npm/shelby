# shelby

The [Shelby Cobra](https://en.wikipedia.org/wiki/AC_Cobra) is an iconic sportscar as well as a monitoring agent monitoring agent for your unix system, written in Rust. Sends json metrics suitable for ingestion by [numbat](https://github.com/numbat-metrics).

## Build

```bash
cargo build
```

## Usage

Shelby accepts one config option, passed in via the `METRICS` env var. This is the location of the numbat collector to send metrics to.

```bash
METRICS=tcp://127.0.0.1:1337 shelby
```

All metrics sent are in the following form:

```json
{
  "host": "example.com",
  "name": "host.start",
  "time": 1486585740403,
  "value": 1
}
```

## Plugins

### `memory`

Sends:

```json
{
  "value": 0.33
  "name": "host.memory"
}
```

### `disk_usage`

Sends two metrics, one for each mount point:

```json
{
  "host": "example.com",
  "name": "host.disk-usage./",
  "time": 1486587398361,
  "value": 0.8535458850761212
}, {
    "host": "example.com",
    "name": "host.inode-usage./",
    "time": 1486587398361,
    "value": 0.0009513050355418086
}
```

The value is capacity used. (Multiply by 100 if you need a percentage used.) Mount points are hard-coded right now as `/` and `/mnt`.

### `heartbeat`

A pulse from the host; value always 1.

```json
{
  "host": "example.com",
  "name": "host.heartbeat",
  "time": 1486587398361,
  "value": 1
}
```

### `load_average`

Sends 15, 5, and 1 minute load averages.

```json
{ "host": "example.com",
  "name": "host.load-average.1",
  "time": 1486587398362,
  "value": 2.69189453125 },
{ "host": "example.com",
  "name": "host.load-average.5",
  "time": 1486587398362,
  "value": 3.35546875 },
{ "host": "example.com",
  "name": "host.load-average.15",
  "time": 1486587398363,
  "value": 3.2265625 }
```

### `netstat`

Sends one metric for each of the following:

```
sockets.ESTABLISHED
sockets.SYN_SENT
sockets.SYN_RECV
sockets.FIN_WAIT1
sockets.FIN_WAIT2
sockets.TIME_WAIT
sockets.CLOSE
sockets.CLOSE_WAIT
sockets.LAST_ACK
sockets.LISTEN
sockets.CLOSING
```


## License
`shelby` is released under the MIT license. See the [LICENSE](https://github.com/opsmezzo/shelby/blob/master/LICENSE)
file for more information.
