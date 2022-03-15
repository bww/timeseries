# Timeseries
A simple command line tool that generates a series of timestamps given start and end dates. If an end date is omitted, the current time is used.

```sh
$ timeseries series --since=2022-03-05T00:00:00Z --until=2022-03-10T00:00:00Z --stride=1d
2022-03-05T00:00:00Z
2022-03-06T00:00:00Z
2022-03-07T00:00:00Z
2022-03-08T00:00:00Z
2022-03-09T00:00:00Z
2022-03-10T00:00:00Z
```

You can express the start and end dates as:

* An RFC 3339 timestamp,
* A relative adjustment, e.g., `+12h30m` or `-90d`,
* A few useful constants: `yesterday`, `today`, `now`, `tomorrow`.

```sh
$ timeseries series --since=-5d --until=now --stride=1d
2022-03-10T03:35:09Z
2022-03-11T03:35:09Z
2022-03-12T03:35:09Z
2022-03-13T03:35:09Z
2022-03-14T03:35:09Z
2022-03-15T03:35:09Z
```

The `--stride` flag can be used to adjust the duration between each timestamp that is output.

```sh
$ timeseries series --since=-5h --stride=30m
2022-03-14T22:35:31Z
2022-03-14T23:05:31Z
2022-03-14T23:35:31Z
2022-03-15T00:05:31Z
2022-03-15T00:35:31Z
2022-03-15T01:05:31Z
...
```
