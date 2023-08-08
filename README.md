# Simple http server for test routing

## Configuration environment variables

- `REPLY__HTTP_SERVER__PORT=8000` Change http server port
- `REPLY__SERVER_NAME` Change server name

## Routes
- `/` Print hostname and server_name
- `/epoch` Prints unix epoch


## Cross build


```shell
cross build --release --target x86_64-unknown-linux-musl
```
