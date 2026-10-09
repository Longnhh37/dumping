# GET /
GET http://127.0.0.1:8000/
HTTP 200
[Asserts]
header "Content-Type" contains "json"
jsonpath "$.status" == "ok"
