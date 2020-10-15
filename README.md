# UDP listener
A simple UDP listener console app.

***

To start:

```bash
./udp-listener
./udp-listener --port 9191 --buffer_size 65515
```

If no port or buffer_size specified, defaults are port=9191, buffer_size=65515 bytes.

***

Start and test with netcat by typing something in another terminal window:
```bash
netcat -u localhost:9191
``` 
