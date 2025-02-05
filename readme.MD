# Database Performance Metrics

A Rust-based service to compare performance metrics across different databases.

## Supported Databases

- MongoDB
- PostgreSQL
- RocksDB
- LevelDB
- SurrealDB

## Performance Results

The following metrics show performance comparisons between different databases for read and write operations with 400 records each:

**Note: Execution time is in seconds**

```js
[
  {
    db_name: "mongodb",
    performance: [
      {
        execution_time: 0.24,
        total_records: 400,
        operation: "Read",
      },
      {
        execution_time: 23.034,
        total_records: 400,
        operation: "Insert",
      },
    ],
  },
  {
    db_name: "postgresql",
    performance: [
      {
        execution_time: 0.132,
        total_records: 400,
        operation: "Read",
      },
      {
        execution_time: 13.571,
        total_records: 400,
        operation: "Insert",
      },
    ],
  },
  {
    db_name: "rocksdb",
    performance: [
      {
        execution_time: 0.045,
        total_records: 400,
        operation: "Read",
      },
      {
        execution_time: 0.0105,
        total_records: 400,
        operation: "Insert",
      },
    ],
  },
  {
    db_name: "leveldb",
    performance: [
      {
        execution_time: 0.269,
        total_records: 400,
        operation: "Read",
      },
      {
        execution_time: 2.925,
        total_records: 400,
        operation: "Insert",
      },
    ],
  },
  {
    db_name: "surrealdb",
    performance: [
      {
        execution_time: 0.532,
        total_records: 400,
        operation: "Read",
      },
      {
        execution_time: 70.856,
        total_records: 400,
        operation: "Insert",
      },
    ],
  },
];
```

These performance measurements should be considered as indicative samples only, as actual performance can vary significantly based on multiple factors:

1. Deployment Environment
2. Database Configuration
3. System Factors
4. Data Characteristics

## APIs

## Endpoints

### 1. Home

- **URL:** `/`
- **Method:** `GET`
- **Description:** Welcome endpoint
- **Response:** Text message welcoming to the db metrics assignment

### 2. Database Metrics

- **URL:** `/metrics`
- **Method:** `GET`
- **Query Parameters:**
  - `db` (string): Database to query metrics for
    - Possible values:
      - `mongodb`
      - `postgresql`
      - `rocksdb`
      - `leveldb`
      - `surrealdb`
      - `all` (returns metrics for all databases)
- **Response Format:** JSON
- **Example Requests:**

  ```bash
  # Get metrics for a specific database
  curl "http://localhost:3000/metrics?db=mongodb"

  # Get metrics for all databases
  curl "http://localhost:3000/metrics?db=all"
  ```
