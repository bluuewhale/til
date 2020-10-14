#Specify a base image
FROM node:alpine

#Install Some dependencies
COPY ./package.json ./
RUN npm install

#Default command
WORKDIR /usr/app
COPY ./ ./ 

CMD ["npm", "start"]