# WildBerries Tech L0 Task

## Usage

- -d, --database `<DATABASE>` – pass database URI or string with parameters and its values, for more information check
  tokio-postgres [documentation](https://docs.rs/tokio-postgres/latest/tokio_postgres/config/struct.Config.html#keys). This option is required for launch.
- -h, --help – print help message

## Features
- Onion architecture
- Uses [deadpool-postgres](https://docs.rs/deadpool-postgres/) to maintain database connections pool
- In-memory cache implemented via HashMap
- Supports repository-pattern to maintain data
- AppState contains repository and services
- AppState shared with Arc
- Logging via [env_logger](https://docs.rs/env_logger/latest/env_logger/) and [log](https://docs.rs/log/latest/log/)
- Model rearranged to third normal form of database ([structure](./migrations/V1__init_up.sql))
