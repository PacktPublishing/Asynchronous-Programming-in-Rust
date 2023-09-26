# Delayserver

Delayserver is a webserver that listens on localhost on port 8080 for incoming
connections and waits for the requested duration before responding. It's good
for simulating slow servers and delayed responses.

## Usage

```
in the delayserver folder run:
cargo run delayserver
```

Alternatively, you can install the 

Delay server works by issuing a http GET request in the format: \"http://localhost:8080/[delay in ms]/[HtmlEncoded meesage]\"\n
On reception, it immidiately reports the following to the console: {Message #} - {delay in ms}: {message}\n.
The server then delays the response for the requested time and echoes the message back to the caller.\n\n
