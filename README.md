<p align="center">
  <img src="/screenshots/Logo.png" alt="NATS WebUI Logo"/>
</p>

NATS-WebUI
==========
NATS-WebUI is a web app for monitoring messages on [NATS](https://nats.io/) publications as well as [NATS Server](https://nats.io/) diagnostics. This project was built to explore building web-backends in Rust and uses the following stack:

- HTTP Server and WebSockets in Rust via [Warp Web Framework](https://github.com/seanmonstar/warp) 
- SQLite DB via [rusqlite](https://github.com/jgallagher/rusqlite)
- VueJS
- HTTP Requests via [reqwest](https://github.com/seanmonstar/reqwest)
- [Rants](https://github.com/davidMcneil/rants) Async NATS client

## Screenshots
![Screenshot 4](/screenshots/screenshot4.png) ![Screenshot 3](/screenshots/screenshot3.png)

## Installation
```docker run -p <port>:80 sphqxe/nats-webui```

## Usage
- Add a server by entering its hostname, port, and monitoring port. The monitoring endpoint is called server-side, so the NATS server host must be resolvable and reachable from the server hosting the WebUI.
- In order to subscribe and receive messages from publications, the subjects must be added to the subject hierarchy on the server dashboard. The hierarchy is represented as a subject tree where each node in the tree contains a single subject token. The editor takes input as a tab-spaced tree. For example, to represent the following subjects:
    ````
    time.us
    time.us.east
    time.us.east.atlanta
    time.eu.east
    time.eu.warsaw
    ````
    Input the subject tree as such:
    ````
    time
      us
        east
          atlanta
      eu
        east
        warsaw
    ````
- Create a client to monitor publications. Once the subjects have been entered as previously stated, they should show up on the right side of the client screen. Select the subjects to subscribe to and click the "link" icon to start receiving messages.

## License
MIT

## Authors
Theodore Lee (@sphqxe)