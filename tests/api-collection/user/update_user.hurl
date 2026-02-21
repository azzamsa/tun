PATCH {{BASE_URL}}/users/1
Content-Type: application/json
{
  "name": "frodo"
}

HTTP 200
[Asserts]
jsonpath "$.name" == "frodo"
