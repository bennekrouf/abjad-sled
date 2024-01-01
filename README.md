# Run

````bash
APP_ENV=local cargo watch -c -w src -x run
APP_ENV=staging cargo watch -c -w src -x run
APP_ENV=production cargo watch -c -w src -x run
```

```
APP_ENV=local cargo run
APP_ENV=staging cargo run
APP_ENV=production cargo run
```
