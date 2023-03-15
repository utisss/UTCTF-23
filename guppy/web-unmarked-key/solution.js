const https = require("https");
const fs = require("fs");
const child_process = require("child_process");

let serverHost = "guppy.utctf.live";
let serverPort = 1134;

let sshProc = null;

const options = {
    key: fs.readFileSync("live/privkey.pem"),
    cert: fs.readFileSync("live/fullchain.pem", "utf-8") + fs.readFileSync("lets-encrypt-r3.pem", "utf-8"),
    SNICallback: function(hostname, cb) { // to test with this on, set tls.connect({hostname})
        console.log("SNI LEAKED (this should not happen)", hostname);
        cb();
    }
};

let server = https.createServer(options, (req, res) => {
    console.log("[" + new Date() + "] " + req.method + " " + req.url);
    res.writeHead(200);
    res.end('hello world\n');
    if (sshProc) {
        // Ctrl-C - just so server doesn't get an EPIPE
        // It doesn't crash the server; I just don't want the spam
        sshProc.stdin.write(Buffer.from([8]));
        setTimeout(() => process.exit(0), 100);
    }
});
server.listen(0, function() {
    let addr = server.address();
    sshProc = child_process.spawn("ssh", ["-R", "443:localhost:" + addr.port, "-p", serverPort + "", serverHost], {
        stdio: ["pipe", "inherit", "inherit"]
    });
    sshProc.on("exit", code => {
        console.log("SSH client exited with " + code);
        setTimeout(() => process.exit(1), 1000);
    });
});
