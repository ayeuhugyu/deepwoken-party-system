import express from "express";
import { Server } from "socket.io";
import http from "http";
import bodyParser from "body-parser";

const password = "123456"; // CHANGE THIS!!!!

const app = express();
const server = http.createServer(app);
const port = 7878;

const io = new Server(server);

let authorized = []

io.on('connection', (socket) => {
    console.log('a user connected');
    socket.on("updateStat", (data) => {
        console.log(data);
        io.emit("updateStat", data);
    });
    socket.on('disconnect', () => {
        console.log('user disconnected');
    });
});

app.use(bodyParser.json());

app.post('/auth', (req, res) => {
    const pass = req.body.password;
    const clientIp = req.headers['x-forwarded-for'] || req.connection.remoteAddress;
    console.log(req.body);
    console.log(`Authenticating ${clientIp} with password ${pass}`);
    if (pass !== password) {
        console.log(`Unauthorized`);
        return res.status(401).send('Unauthorized');
    }
    authorized.push(clientIp);
    console.log(`Authorized ${clientIp}`);
    res.send(`Authenticated ${clientIp} to DWPS server`);
});

app.get('/', (req, res) => {
    res.send('Hello World!');
});

server.listen(port, () => {
    console.log(`Server is running on http://localhost:${port}`);
});