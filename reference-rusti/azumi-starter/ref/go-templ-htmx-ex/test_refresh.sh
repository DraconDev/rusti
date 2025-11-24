curl -X POST "http://localhost:8081/api/auth/refresh" \
  -H "Content-Type: application/json" \
  -b "/tmp/test_cookies.txt" \
  -c "/tmp/test_cookies.txt" \
  -v