# Raspberry Pi DNS

The Raspberry Pi has had my interest since version 5 was released. They seem to be getting more powerful without compromising the size. To kick off the new year, I wanted to create a more private internet environment for my home, and the Raspberry Pi is the perfect device for that. As a bonus, this was a great opportunity to learn about the DNS protocol, IP addresses, local networks, and more.

## Prerequisites

Before the magic could happen, I needed to do a little bit of lift on the setup part. Out of the box, the Raspberry Pi is just a blank green card ready to be plugged in. First step was to grab a 64gb microSD card I had lying around and get the OS flashed onto it. There are plenty of guides out there to help with the setup process. Setting up the home wifi connection and enabling SSH were the key parts. After flashing the OS, I slapped the card in the Pi and plugged it up. All green and ready to go. Using VScode, I was able to SSH into the Pi and start getting things set up.

### IP Address

What is the IP address of the Pi? This is when LAN and WAN started to swirl. Using some handy cli commands, the IP address was found. But why is this string of numbers important? Well, the IP address is a unique identifier for the device on the network. But what network? Surely nobody could just type this IP address in and get access to my Pi right? Further research concluded that no, this is my local networks IP. LAN is the local area network, and WAN is the wide area network. The IP address is the unique identifier for the device on my network that is within my routers range. We also need this to be a static one and not dynamically assigned, and this can be done through the admin interface of the router.

## What is DNS

DNS is a network protocol that is used to resolve domain names to IP addresses. It is used to map domain names to IP addresses, making it possible for machines on the internet to find each other easily. Our human friendly URLs are mapped to IP addresses using DNS as the middle man. This is usually handled by a third party such as Google or Cloudflare. The issue is that I cannot configure (for the most part) these third party DNS services. So I need to configure my own DNS server.

## Setting up a DNS Server

To set up a DNS server, I need to install and configure the DNS server. Thanks to open source projects, I can use the well regarded Pi-Hole software. It is a DNS server that is easy to set up and has a simple user interface.

### Install Pi-Hole

Docker is my preferred method for running almost anything, as long as I can get the container to be small enough to match the benefit. Luckily, Pi-Hole has the instructions for setting it up with Docker and they even have a docker compose example using nginx.

```yml:
  nginx-proxy:
    image: nginxproxy/nginx-proxy
    ports:
      - '80:80'
    environment:
      DEFAULT_HOST: pihole.yourDomain.lan
    volumes:
      - '/var/run/docker.sock:/tmp/docker.sock'
    restart: always

  pihole:
    image: pihole/pihole:latest
    ports:
      - '53:53/tcp'
      - '53:53/udp'
      - "67:67/udp"
      - '8053:80/tcp'
    volumes:
      - './etc-pihole:/etc/pihole'
      - './etc-dnsmasq.d:/etc/dnsmasq.d'
    cap_add:
      - NET_ADMIN
    environment:
      FTLCONF_LOCAL_IPV4: <my-raspberry-pi-ip> 
      PROXY_LOCATION: pihole
      VIRTUAL_HOST: pihole.yourDomain.lan
      VIRTUAL_PORT: 80
    extra_hosts:
      - 'nw2master.bioware.com nwn2.master.gamespy.com:0.0.0.0'
      - 'yourDomain.lan:<my-raspberry-pi-ip>'
      - 'pihole pihole.yourDomain.lan:<my-raspberry-pi-ip>'
      - 'ghost ghost.yourDomain.lan:<my-raspberry-pi-ip>'
      - 'wordpress wordpress.yourDomain.lan:<my-raspberry-pi-ip>'
    restart: always
```

I was able to just plug in the IP address assigned to my Pi and run `docker compose up`. After a few minutes, I was able to browse to the Pi-Hole dashboard from my browser by pasting in the Pi's IP address.

### Setting Up the Router

From the same admin panel used to configure a static IP for our Pi, I was able to configure the router to use the Pi's IP address as the DNS server. This is were the magic happens. When our devices talk to our router to get the IP address of a website, the router will use the Pi's IP address, where our DNS server lives (port 80) to do so.

## Using Pi-Hole

So now that the DNS server is running on the Pi, the setup can be completed through the dashboard. The most popular thing to do is to add "Blocklists" to the DNS database to block unwanted content. There are a bunch of great lists online to achieve this and they are simply URLs that point to txt files that people created. It did take an hour or so to get the right mix, but it was definitely worth it. I was even able to add very specific URLs after using the service for a few days, such as Hulu ads. It was a great learning experience.

## What Is Happening

This seemed to easy to be true, but it was indeed one of the rare times were it was just that easy. So what is going on here?

1. Our devices are connected to the same network as the Pi. That is the Local Area Network (LAN).
2. When we browse to a website, the request has to go to a DNS server first to get the IP address of the website.
3. The DNS server then sends the request to the IP address of the website. We now have that DNS server on our LAN, and under our control.
4. When one of our devices are connected to the same network as the Pi, the Pi becomes our DNS server.
5. From Pi-Holes user interface, we can control what get's blocked and what doesn't.

And by blocked, that means the DNS will simply return a blank IP address. In other words, the Pi will not forward any requests to the IP address of the website or service. So when your smart TV tries to send the data it captured from your viewing habits, it will not be able to do so.

## Conclusion

Learning how to configure a Raspberry Pi and how local networks operate was a fun experience. On top of that, I get to use this in my day to day life.
