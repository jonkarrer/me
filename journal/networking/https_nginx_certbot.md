# Networks

Notes on various methods and tasks to configure a network and domain.

## Domains / HTTPS

Two things need to be done for an https connection

1. DNS provider needs to be pointed at the IP address of the application
   - **A Record** needs the **Value** to be the IP address
   - Example: AWS instance "devjon" has the IP of 34.225.144.203 and "theprep.app" has an A Record pointed at that IP.
2. The application needs to be listening on port 80 and 443 for incoming requests, and have certificates ready.
   - Nginx is being used as the reverse proxy for this. It can handle most of the lift for any app.

## Nginx

Nginx needs to be installed an configured on the machine.

### Example

For linux based machines run

```bash
sudo apt update
sudo apt install nginx
```

Next, the server block needs to be created for the domain of the application. Go to _/etc/nginx/sites-available_ and create a file for the domain configuration.

```bash
cd /etc/nginx/sites-available && touch exampledomain.com
```

Open the text editor for this file and configure the server block.

```nginx
server {
    listen 443 ssl http2;
    server_name exampledomain.com www.exampledomain.com;

    ssl_certificate /etc/letsencrypt/live/exampledomain.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/exampledomain.com/privkey.pem;

    location / {
        proxy_pass http://localhost:8000; # point this at the port where the app is listening
        include /etc/nginx/proxy_params;
    }

    location = /health {
        access_log off;
        add_header 'Content-Type' 'application/json';
        return 200 '{"status":"UP"}';
    }
}
```

Edit the default file config in _/etc/nginx/sites-available_ to redirect traffic

```nginx
server {
    listen 80 default_server;
    server_name _;

    location /healthz {
        access_log off;
        return 200 'ok';
        add_header Content-Type text/plain;
    }

    location / {
        return 301 https://$host$request_uri;
    }
}
```

Now symlink the new domain config **exampledomain.com** to the directory _/etc/nginx/sites-enabled_ with this command

```bash
sudo ln -s /etc/nginx/sites-available/exampledomain.com /etc/nginx/sites-enabled/
```

Test the new configs with

```bash
sudo nginx -t
```

And if all goes well restart nginx

```bash
sudo systemctl reload nginx
```

Now do a health check. Should get a 200 ok response from the default server block listening on port 80.

```bash
curl http://exampledomain.com
```

## Certbot

Now we need to provision the certificate for **exampledomain.com**. Cerbot is the easy to use tool from [letsencrypt](https://letsencrypt.org/).

### Certbot Example

For linux based machines run

```bash
sudo apt install certbot python3-certbot-nginx
```

Port 80 will be needed for this to work, so we have to stop nginx or whatever service is listening on port 80.

```bash
# check to see if port 80 is available.
sudo ss -tulnp | grep :80

# Kill nginx process (maybe)
sudo systemctl stop nginx

# Kill process by PID
kill -9 <pid>
```

Now we need to do a _Dry Run_ with certbot. This will prevent timeouts from letsencrypt if we have issues.

```bash
sudo certbot certonly --dry-run -d exampledomain.com -d www.exampledomain.com
```

If that goes well, time do do a real run.

```bash
sudo certbot certonly --standalone -d exampledomain.com -d www.exampledomain.com
```

Certbot will give the path that has the certs, usually **/etc/letsencrypt/live/exampledomain.com/**. We already linked this path in our nginx config above, so all that's left to do is test.

First, restart nginx

```bash
sudo systemctl start nginx
```

Now curl for the new <https://exampledomain.com/health> endpoint.

```bash
curl https://brize.dev/health
```

Should get

```json
{ "status": "UP" }
```

Renewing the cert will be taken care of by Certbot. To manage these automatic timers, find them with

```bash
systemctl list-timers | grep certbot
```

The we can stop or disable them with

```bash
sudo systemctl stop certbot.timer && sudo systemctl disable certbot.timer
```

Additionally, crontab can be used to make a custom job run. Open the editor with this command

```bash
crontab -e
```

Use a tool to calculate the crontab syntax, then write it in the file. Here is a 3 month job example.

```bash
0 0 1 */3 * sudo certbot renew
```

## Docker Compose Nginx and Certbot

[Blog Post](https://mindsers.blog/en/post/https-using-nginx-certbot-docker/)

### Phase 1

Set up docker-compose.yml

```yml
version: "3.8"

services:
  nginx:
    image: nginx:latest
    ports:
      - 80:80
      - 443:443
    restart: always
    volumes:
      - ./nginx/conf/:/etc/nginx/conf.d/:ro
      - ./certbot/www:/var/www/certbot/:ro
  certbot:
    image: certbot/certbot:latest
    volumes:
      - ./certbot/www/:/var/www/certbot/:rw
```

Set up nginx.conf

```conf
server {
    listen 80;
    server_name theprep.app www.theprep.app;
    server_tokens off;

    location /.well-known/acme-challenge/ {
        root /var/www/certbot;
    }

    location / {
        return 301 https://theprep.app$request_uri;
    }
}
```

Do a dry run

```bash
docker compose --env-file .env.prod -f docker-compose.prod.yml run --rm  certbot certonly --webroot --webroot-path /var/www/certbot/ --dry-run -d theprep.app
```

If this goes well, move to phase 2

### Phase 2

Start the web service

```bash
docker compose --env-file .env.prod -f docker-compose.prod.yml -d up web
```

Add to docker-compose.yml

```yml
nginx:
    image: nginx:latest
    ports:
      - 80:80
      - 443:443
    restart: always
    volumes:
      - ./proxy/nginx.conf:/etc/nginx/conf.d/default.conf
      - ./certbot/www:/var/www/certbot/:ro
      - ./certbot/conf/:/etc/nginx/ssl/:ro

  certbot:
    image: certbot/certbot:latest
    volumes:
      - ./certbot/www/:/var/www/certbot/:rw
      - ./certbot/conf/:/etc/letsencrypt/:rw
```

Add to nginx.conf

```conf
upstream loadbalancer {
  server web:8000;
}

server {
    listen 443 default_server ssl http2;
    server_name theprep.app www.theprep.app;

    ssl_certificate /etc/nginx/ssl/live/theprep.app/fullchain.pem;
    ssl_certificate_key /etc/nginx/ssl/live/theprep.app/privkey.pem;

    location / {
        proxy_pass http://loadbalancer;
    }

    location = /health {
        access_log off;
        add_header 'Content-Type' 'application/json';
        return 200 '{"status":"UP"}';
    }
}

server {
    listen 80;
    server_name theprep.app www.theprep.app;
    server_tokens off;

    location /.well-known/acme-challenge/ {
        root /var/www/certbot;
    }

    location / {
        return 301 https://theprep.app$request_uri;
    }
}
```

Restart nginx container to pick up changes

```bash
docker compose --env-file .env.prod -f docker-compose.prod.yml restart nginx
```

Do a real certbot run

```bash
docker compose --env-file .env.prod -f docker-compose.prod.yml run --rm  certbot certonly --webroot --webroot-path /var/www/certbot/ -d theprep.app
```

Restart nginx again to pick up the real certs

```bash
docker compose --env-file .env.prod -f docker-compose.prod.yml restart nginx
```

See if the ports are up

```bash
sudo ss -tulnp | grep :443
sudo ss -tulnp | grep :80
```

Everything should be working.
