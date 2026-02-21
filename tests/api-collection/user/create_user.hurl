POST {{BASE_URL}}/users
Content-Type: application/json
{
  "name": "aragorn"
}

HTTP 201
[Asserts]
jsonpath "$.name" == "aragorn"
