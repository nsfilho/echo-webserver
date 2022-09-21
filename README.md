# Objective

The main objective of this app is to echo some informations via http to validate if others app can communicate with this server.

Some fields, like `unique_id` and `server_time` shows you about if your connection is intercepted by a cache or not and if your time and server is properly configured.

## Running

```
docker run --name echo -p 8000:8000 -d nsfilho/echo-webserver:latest
```

## Testing

Querying info:

```
curl --location --request GET 'http://127.0.0.1:8000/'
```

Response from server:

```
{ sample json }
```

