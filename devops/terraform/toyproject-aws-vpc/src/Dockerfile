FROM golang:alpine as Builder
MAINTAINER Donghyung Ko <koko8624@pubg.com>

WORKDIR /src
ENV CGO_ENABLED=0 \
    GOOS=linux \
    GOARCH=amd64 
COPY go.mod main.go ./
RUN go build -a -ldflags '-s' -o main main.go

FROM scratch
WORKDIR /usr/src/app
COPY --from=Builder /src/main .
CMD ["./main"]
