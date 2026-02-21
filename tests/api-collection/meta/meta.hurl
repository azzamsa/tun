GET {{BASE_URL}}/meta

HTTP/1.1 200
[Asserts]
status == 200
jsonpath "$.build_date" == "unknown"
jsonpath "$.build_hash" == "unknown"
