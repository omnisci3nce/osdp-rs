```
platform-agnostic
-----------------
parser
packet

platform-specific
-----------------
tokio ? on PC
busy-loop?

Overall control flow - pseudocode

populate fixed sized header

if SCB bit then we populate scb other wise skip to data

data is either fixed-length or variable
if fixed length we just look for X number of bytes
else if variable we continue reading and extending expected data length
based on data in message

validation
calc checksum or crc
```
