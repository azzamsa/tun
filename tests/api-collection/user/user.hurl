GET {{BASE_URL}}/users/1

HTTP 404
[Asserts]
jsonpath "$.message" == "User not found."
