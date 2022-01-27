# gRPC handbook

## grpc reflection (gRPC service 정보 조회용)

gRPC Server Reflection provides information about publicly-accessible gRPC services on a server, and assists clients at runtime to construct RPC requests and responses without precompiled service information. It is used by gRPC CLI, which can be used to introspect server protos and send/receive test RPCs.

```go

import "google.golang.org/grpc/reflection"

        s := grpc.NewServer()
        pb.RegisterGreeterServer(s, &server{})
+       // Register reflection service on gRPC server.
+       reflection.Register(s)
        if err := s.Serve(lis); err != nil {
                log.Fatalf("failed to serve: %v", err)
        }
```

### Check reflection for gRPC CLI

```bash
git clone https://github.com/grpc/grpc
cd grpc
git submodule update --init
make grpc_cli
cd bins/opt # grpc_cli is in directory bins/opt/
```

```bash
./grpc_cli ls localhost:50051

# Result
grpc.examples.echo.Echo
grpc.reflection.v1alpha.ServerReflection
helloworld.Greeter

# List services and method
./grpc_cli ls localhost:50051 helloworld.Greeter -l

# Result
filename: helloworld.proto
package: helloworld;
service Greeter {
  rpc SayHello(helloworld.HelloRequest) returns (helloworld.HelloReply) {}
}

# Inspect message types
./grpc_cli type localhost:50051 helloworld.HelloRequest

# Result
message HelloRequest {
  string name = 1[json_name = "name"];
}

# Call a remote method
./grpc_cli call localhost:50051 SayHello "name: 'gRPC CLI'"

# Result
connecting to localhost:50051
message: "Hello gRPC CLI"

Rpc succeeded with OK status
```

## gRPC Encryption (gRPC communication channel encryption)

### Terms

- `CA (Certificate Authority)`: 인증기관
- `PEM (Privacy Enhanced Mail)`: certificate info + public key encoded by base64
- `CSR (Certificate Signing Request)`: CA 인증서 생성 요청을 위한 파일
- `CRT (CeRTificate)`: CA가 증명하는 인증서
- `$SERVER.key`: server private key
- `$SERVER.pem`: PKCS#8 certificate with server public key
- `$SERVER.csr`: server CSR (Certificate Signing Request) file
- `$SERVER.crt`: server certificate file (서버 인증서)
- `$ROOTCA.key`: CA private key
- `$ROOTCA.crt`: CA certificate file for `$SERVER.crt` (`$SERVER.pem`)

### Generate ceritificate (인증서 성성)

```bash
openssl genrsa -out ca.key 2048
openssl req -new -x509 -days 365 -key ca.key -subj "/C=KR/L=AY/O=HFR,Inc./CN=HFR's Self Signed CA" -out ca.crt

openssl req -newkey rsa:2048 -nodes -keyout service.key -subj "/C=KR/L=AY/O=HFR,Inc./CN=HFR NE" -out service.csr
openssl x509 -req -extfile <(printf "subjectAltName=DNS:localhost") -days 365 -in service.csr -CA ca.crt -CAkey ca.key -CAcreateserial -out service.crt


$ openssl genrsa -out server.key 2048

# crt 바로 생성
$ openssl req -new -x509 -sha256 -key server.key \
              -out server.crt -days 3650
# csr -> crt 으로 변환
$ openssl req -new -sha256 -key server.key -out server.csr
$ openssl x509 -req -sha256 -in server.csr -signkey server.key \
               -out server.crt -days 3650
$ openssl req -new -x509 -sha256 -key server.key -out server.crt -days 36500 -subj "/C=KR/ST=Gyeonggi/O=HFR,Inc./CN=HFR NE"
```

```bash
# Create ROOT certificate (TSL/SSL 인증서 생성하기)

# The name of ROOT CA (Certificate Authority)
export ROOTCA="HFR's Self Signed CA"
# Name of SERVER
export SERVERCA="HFR NE"
# Generated file name
export SERVER="server"

# Generate ROOT CA Private key (rootca.key)
openssl genrsa -aes256 -out rootca.key 2048

# Create ROOT certificate (rootca.crt) of ROOT CA (인증서 생성)
openssl req -new -x509 -days 36500 -key rootca.key -out rootca.crt -subj "/C=KR/ST=Gyeonggi/O=HFR,Inc./CN=$ROOTCA"

# check ROOT certificate (인증서 확인)
openssl x509 -text -in rootca.crt


# Create Server certificate

# Generate server Private key ($SERVER.key)
openssl genrsa -aes256 -out $SERVER.key 2048

# Create Certificate request ($SERVER.csr)
openssl req -new -key $SERVER.key -out $SERVER.csr -subj "/C=KR/ST=Gyeonggi/O=HFR,Inc./CN=$SERVERCA"

# Create certificate (digital certificate file; 인증서)
openssl x509 -req -days 36500 -in $SERVER.csr -CA rootca.crt -CAkey rootca.key -set_serial 01 -out $SERVER.crt

# Create PKCS#8 certificate for gRPC
# openssl pkcs8 -topk8 -nocrypt -in $SERVER.key -out $SERVER.pem


```

### Generate crt without asking subjectAltName

```bash
openssl req -new -sha256 \
    -key domain.key \
    -subj "/C=US/ST=CA/O=Acme, Inc./CN=example.com" \
    -reqexts SAN \
    -config <(cat /etc/ssl/openssl.cnf \
        <(printf "\n[SAN]\nsubjectAltName=DNS:example.com,DNS:www.example.com")) \
    -out domain.csr
```

### another example

```bash
# Generate CA key:
openssl genrsa -des3 -out ca.key 4096

# Generate CA certificate:
openssl req -new -x509 -days 365 -key ca.key -out ca.crt

# Generate server key:
openssl genrsa -des3 -out server.key 4096

# Generate server signing request:
openssl req -new -key server.key -out server.csr

# Self-sign server certificate:
openssl x509 -req -days 365 -in server.csr -CA ca.crt -CAkey ca.key -set_serial 01 -out server.crt

# Remove passphrase from the server key:
openssl rsa -in server.key -out server.key

# Generate client key:
openssl genrsa -des3 -out client.key 4096

# Generate client signing request:
openssl req -new -key client.key -out client.csr

# Self-sign client certificate:
openssl x509 -req -days 365 -in client.csr -CA ca.crt -CAkey ca.key -set_serial 01 -out client.crt

# Remove passphrase from the client key:
openssl rsa -in client.key -out client.key
```