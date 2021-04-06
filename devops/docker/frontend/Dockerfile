# Builder-phase
FROM node:alpine as builder

WORKDIR "/app"
COPY package.json .
RUN npm install
COPY . .
RUN npm run build

# Deploying-phase
FROM nginx
COPY --from=builder /app/build /usr/share/nginx/html
