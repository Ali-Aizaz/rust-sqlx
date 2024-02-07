# Setup Database

In order to setup the database, you can use the following command with Docker installed:

```$ docker run -e POSTGRES_PASSWORD=mysecretpassword -e POSTGRES_USER=dbuser -e POSTGRES_DB=bookstore  -p 5432:5432 postgres:1```

If you don't want to install docker just use a free test database provider vercel
