# REST API health
GET http://127.0.0.1:8000/health

HTTP/1.1 200
[Asserts]
status == 200
jsonpath "$.status" == "running"

# meta
GET http://127.0.0.1:8000/meta

HTTP/1.1 200
[Asserts]
status == 200
jsonpath "$.build" == "unknown"
