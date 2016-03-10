## Siege Results

The endpoint is a JSON response of 10 rows of data randomly pulled from a four column MySQL table. The table contains ~100,000 rows.

### Run #1
**Command:** `siege -b -t60S  http://localhost:9001/raw`

**Notes:** MySQL connection within the server routing function

```
Transactions:            668 hits
Availability:         100.00 %
Elapsed time:          59.60 secs
Data transferred:         0.46 MB
Response time:            1.32 secs
Transaction rate:        11.21 trans/sec
Throughput:           0.01 MB/sec
Concurrency:           14.80
Successful transactions:         668
Failed transactions:             0
Longest transaction:          2.03
Shortest transaction:         0.67
```

### Run #2
**Command:** `siege -b -t60S  http://localhost:9001/raw`

**Notes:** MySQL connection outside the server routing function

```
Transactions:           1320 hits
Availability:         100.00 %
Elapsed time:          59.82 secs
Data transferred:         0.90 MB
Response time:            0.68 secs
Transaction rate:        22.07 trans/sec
Throughput:           0.02 MB/sec
Concurrency:           14.91
Successful transactions:        1320
Failed transactions:             0
Longest transaction:          1.00
Shortest transaction:         0.34
```

### Run #3

**Command:** `siege -c50 -d10 -t60S  http://localhost:9001/raw`

**Notes:** MySQL connection outside the server routing function with a concurrency of 50

```
Transactions:            577 hits
Availability:         100.00 %
Elapsed time:          59.19 secs
Data transferred:         0.39 MB
Response time:            0.14 secs
Transaction rate:         9.75 trans/sec
Throughput:           0.01 MB/sec
Concurrency:            1.40
Successful transactions:         577
Failed transactions:             0
Longest transaction:          0.34
Shortest transaction:         0.11
```
