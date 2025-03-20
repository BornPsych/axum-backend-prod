## Dev (REPL)

> NOTE: Install baocn with `cargo install bacon`.

```sh
# Terminal 1 - To run the server.
bacon run-long -- -q

# Terminal 2 - To run the quick_dev.
bacon ex -- -q --example quick_dev
```

## Starting the DB

```sh
# Start postgresql server docker image:
docker run --rm --name pg -p 5432:5432 \
   -e POSTGRES_PASSWORD=welcome \
   postgres:15

# (optional) To have a psql terminal on pg. 
# In another terminal (tab) run psql:
docker exec -it -u postgres pg psql

# (optional) For pg to print all sql statements.
# In psql command line started above.
ALTER DATABASE postgres SET log_statement = 'all';
```


## Dev

```sh
# Terminal 1 - To run the server.
cargo run

# Terminal 2 - To run the tests.
cargo run --example quick_dev
```
