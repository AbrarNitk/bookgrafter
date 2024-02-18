# Task Update

- axum setup
  - reading the environment vars
  - starting the server
  - define the router and apis which are going to call from the cli
- cli commands
  - serve
  - asking question and saving the answers

## curl: To ask Question
  
```shell
curl -X POST --header "Content-Type: application/json" \
--data '{"query": "Hello World program in Rust?", "topic": "rust/hello"}' \
http://127.0.0.1:8000/-/v1/api/chat/gemini/
```