# forza
The Monitoring Agent.

## Build
```bash
cargo build
```
## Usage
```bash
METRICS=tcp://127.0.0.1:1337 forza
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

## License
`forza` is released under the MIT license. See the [LICENSE](https://github.com/opsmezzo/forza/blob/master/LICENSE)
file for more information.
