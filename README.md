# Auth Service

This is microservice with event driven architecture built on top of actix web and it's really really fast and scalable with SeaORM postgresql and kafka.

<img align=center width="1001" alt="Screenshot 2023-12-04 at 08 25 00" src="https://github.com/Geriano/nightmare-auth/assets/59258929/022aad8a-72fc-49af-8a6b-8a709e25d1a1">

### Installation
```Bash
docker compose up
```

### Migration
```Bash
docker exec -it nightmare-auth-app sea migrate fresh
```

Boom! your service already served, for openapi page you can just move it to /doc on your browser.

![image](https://github.com/Geriano/nightmare-auth/assets/59258929/08f1403c-aae8-43f8-b0f7-d261e4409283)

Database structure

<img align=center width="1076" alt="Screenshot 2023-12-04 at 08 13 15" src="https://github.com/Geriano/nightmare-auth/assets/59258929/93bf96bd-9a57-455d-afe0-9b7931864874">
