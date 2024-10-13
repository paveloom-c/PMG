FROM docker.io/nginx:1.27.2-alpine-slim

COPY landing /usr/share/nginx/html
