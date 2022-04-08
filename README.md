

curl -v http://localhost:8000/auth/login

curl -v -X POST http://localhost:8000/auth/login

curl -v -X POST http://localhost:8000/auth/login -H "Content-type: application/json" -d '{"response_type": "token"}'

curl -v -X POST http://localhost:8000/auth/login -H "Content-Type: application/x-www-form-urlencoded" -d 'response_type=token'



curl -v -X POST http://localhost:8000/auth/login -H "Content-type: application/json" -d '{"response_type": "code"}'


curl -v -X POST http://localhost:8000/auth/login -H "Content-type: application/json" -d '{"response_type": "code", "redirect_uri": "http://demo"}'

curl -v -X POST http://localhost:8000/auth/login -H "Content-type: application/json" -d '{"response_type": "code", "redirect_uri": "http://demo", "scope": "read,write"}'


curl -v -X POST http://localhost:8000/auth/login -H "Content-Type: application/x-www-form-urlencoded" -d 'response_type=code'
curl -v -X POST http://localhost:8000/auth/login -H "Content-Type: application/x-www-form-urlencoded" -d 'response_type=code&redirect_uri=http://demo'
