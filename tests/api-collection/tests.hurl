# REST API health
GET {{BASE_URL}}/health

HTTP/1.1 200
[Asserts]
status == 200

# meta
GET {{BASE_URL}}/meta

HTTP/1.1 200
[Asserts]
status == 200
jsonpath "$.build" == "unknown"
