FROM docker.io/nginx:1.27.2-alpine-slim

LABEL org.opencontainers.image.source=https://github.com/paveloom-c/PMG

COPY landing /usr/share/nginx/html
