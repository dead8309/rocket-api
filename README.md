# rocket-api

Experimenting with Rust and Rocket web framework. This is a simple REST Api that interacts with a MongoDB database to perform CRUD operations on a User model. 

## Endpoints

- GET /users
- GET /users/:id
- POST /users
- PUT /users/:id
- DELETE /users/:id

## How to run

### Prerequisites

- Rust
- Cargo
- Docker

### Steps

1. Clone the repository
```bash
git clone https://github.com/dead8309/rocket-api.git
```

2. Start the mongodb container
```bash
docker compose up -d mongodb
```

3. Start the api server 
```bash
docker compose up -d rocketapi
```