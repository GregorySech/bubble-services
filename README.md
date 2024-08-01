# Bubble Services
A digital office for anagraphic services.

[Documentation](https://gregorysech.github.io/bubble-services/)

# Features
## Call requests
An unauthenticated user can request to be called by providing (at least) their phone number.

An authenticated office-worker will find call requests in their dashboard. From there they can mark them as done.

# Development setup
A base configuration can be found inside the `configuration` folder.
To get sqlx to work locally you will need a running postgres database and define the connection url inside a `.env` file under the `DATABASE_URL`, follows an example.

```bash
DATABASE_URL="postgres://postgres:password@localhost:5432/bubble_services"
```

For preparing sqlx for offline compilation of all binary targets (for now tests) run: `cargo sqlx prepare -- --all-targets `.
