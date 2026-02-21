GET {{BASE_URL}}/users?

HTTP 200
[Asserts]
jsonpath "$" isEmpty
