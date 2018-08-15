# NTP Drift Logger

This program allows you to track you system clock drift over time using a NTP
server.

Just run `ntp-drift-logger [server:port] [samples] [filename]` and the program
will start querying the server at the given port every 30 seconds, taking the
average of the given number of samples and saving all this data in the specified
file. 

## CSV example

```csv
timestamp,offset,delay
2018-08-16 00:14:12.048420 +02:00,276,627
2018-08-16 00:14:43.172863 +02:00,211,552
2018-08-16 00:15:13.928976 +02:00,237,540
2018-08-16 00:15:45.016705 +02:00,242,599
```
