# Delayserver

Delayserver is a web server that listens on localhost port 8080 for incoming
connections and waits for the requested duration before responding. It's good
for simulating slow servers and delayed responses.

## Usage

In the delayserver folder run:

```
cargo run delayserver
```

You can alternatively install the program locally so it's always available in PATH:

```
cargo install --path .
```

Delay server works by issuing a http GET request in the format:

```
http://localhost:8080/[delay in ms]/[UrlEncoded meesage]
```

On reception, it immediately reports the following to the console:

```
{Message #} - {delay in ms}: {message}
```

The server then delays the response for the requested time and echoes the message back to the caller.

Please note that the message must be **Url** encoded (i.e. a space is encoded as `%20`).
