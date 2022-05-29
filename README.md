# datafusion-tui (dft)

DataFusion-tui provides an extensible terminal based data analysis tool that uses [DataFusion](https://github.com/apache/arrow-datafusion) (single node) and [Ballista](https://github.com/apache/arrow-ballista) (distributed) as query execution engines. It has drawn inspiration and several features from `datafusion-cli`. In contrast to `datafusion-cli` a focus of `dft` is to provide an interface for leveraging DataFusions extensibility (for example connecting to `ObjectStore`s or querying custom `TableProvider`s).  

The objective of `dft` is to provide users with the experience of having their own local database that allows them to query and join data from disparate data sources all from the terminal.  

https://user-images.githubusercontent.com/622789/161690194-c7c1e1b0-e432-43ab-9e44-f7673868b9cb.mp4

Some of the current and planned features are:

- Tab management to provide clean and structured organization of DataFusion queries, results, and context
  - SQL editor
    - Text editor for writing SQL queries
    - Scrollable query results
    - Track memory usage during query (TODO)
    - Write query results to file (TODO)
    - Multiple SQL Editor tabs (TODO)
  - Query history
    - History of executed queries
  - ExecutionContext information
    - Information from ExecutionContext / Catalog / ObjectStore / State / Config
  - Logs
    - Logs from `dft` and `DataFusion`
- `ObjectStore` Support
  - S3 with AWS default credentials
  - S3 with custom endpoint / provider (i.e. MinIO)
  - HDFS (TODO)
  - `ObjectStore` explorer. I.e. able to list files in `ObjectStore`
  - There are ongoing conversations in DataFusion about adopting a new `ObjectStore` interface that would come with bindings to S3, ADLS, and GCP.  I am  monitoring this and plan on updating to use that interface when it is available. 
- `TableProvider` data sources
  - Delta Table => TODO
  - Google Big Table => (currently in the bigtable branch which isnt up to date with latest DataFusion )
  - ApiTable => Will allow treating API endpoints as tables by handling pagination and authentication.  Currently being prototyped in [#85](https://github.com/datafusion-contrib/datafusion-tui/pull/85)
- Preloading DDL from `~/.datafusion/.datafusionrc` for local database available on startup

## User Guide

To have the best experience with `dft` it is highly recommended to define all of your DDL in `~/.datafusion/.datafusionrc` so that any tables you wish to query are available at startup.  Additionally, now that DataFusion supports `CREATE VIEW` via sql you can also make a `VIEW` based on these tables.

The interface is split into several tabs so that relevant information can be viewed and controlled in a clean and organized manner. When not writing a SQL query keys can be entered to navigate and control the interface.

- SQL Editor: where queries are entered and results can be viewed.  Drawing inspiration from vim there are multiple modes.
  - Normal mode
    - `q` => quit datafusion-tui
    - `e` => start editing SQL Editor in Edit mode
    - `c` => clear contents of SQL Editor
    - `Enter` => execute query
    - Enter the tab number in brackets after a tabs name to navigate to that tab
    - If query results are longer or wider than screen, you can use arrow keys to scroll
  - Edit mode
    - Character keys to write queries
    - Backspace / tab / enter work same as normal
    - `esc` to exit Edit mode and go back to Normal mode
  - Rc mode
    - `l` load `~/.datafusion/.datafusionrc` into editor
    - `r` rerun `~/.datafusion/.datafusionrc`
    - `w` write editor contents to `~/.datafusion/.datafusionrc`
- Register custom `ObjectStore`
  - S3: run / install with `--features=s3`
    - If you want to use your default AWS credentials, then no further action is required. For example your credentials in `~/.aws/credentials` will automatically be picked up.
    - If you want to use a custom S3 provider, such as MinIO, then you must create a `s3.json` configuration file in `~/.datafusion/object_stores/` with the fields `endpoint`, `access_key_id`, and `secret_access_key`.
- Use Ballista as execution engine
  -  Run / install with `--features=ballista` and provide `--host` and `--port` of the ballista scheduler when running `dft`
