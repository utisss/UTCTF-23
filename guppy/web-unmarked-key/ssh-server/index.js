const ssh = require("ssh2");
const { clearInterval } = require("timers");
const https = require("https");
const tls = require("tls");
const readline = require("readline");

const TLS_HOSTNAME = "u-please-t-hack-c-this-t-site-f-2023.mooo.com";
const PING_URL = "/submit?flag=" + process.env.FLAG;
const PORT = 2222;

new ssh.Server({
    hostKeys: [require("fs").readFileSync("server_host_key")]
}, function(client, clientInfo) {
    function logForClient(...args) {
        console.log("[" + clientInfo.ip + "/" + clientInfo.port + "]", ...args);
    }
    logForClient("Connected");
    function wrapTryCatchLogForClient(func) {
        return function(...args) {
            try {
                return func.apply(this, args);
            } catch (e) {
                console.error("CRASH [" + clientInfo.ip + "]", e.stack || e);
                client.end();
            }
        }
    }
    function wrapOnForLogging(obj) {
        if (obj !== null && obj !== undefined && typeof obj.on == "function") {
            let oldOn = obj.on;
            obj.on = function(evt, handler, ...extra) {
                return oldOn.call(this, evt, wrapTryCatchLogForClient(handler), ...extra);
            };
        }
        return obj;
    }
    wrapOnForLogging(client);
    client.on("authentication", function(ctx) {
        // TODO
        logForClient("Authentication approved");
        ctx.accept();
    });
    client.on("error", function(e) {
        logForClient(e);
        client.end();
    });
    client.on("ready", function() {
        let shellRL = null;
        client.on("session", function(accept, reject) {
            let session = accept();
            wrapOnForLogging(session);
            session.on("exec", function(accept, reject, info) {
                logForClient("direct exec:", info.command);
                let channel = accept();
                let rl = readline.createInterface(channel, channel, undefined, true);
                rl.write("Error: Filesystem corruption detected, unable to launch " + info.command + "\r\n");
                channel.exit(127);
                rl.close();
            });
            session.on("shell", function(accept, reject) {
                logForClient("shell");
                let channel = accept();
                let rl = readline.createInterface(channel, channel, undefined, true);
                wrapOnForLogging(rl);
                wrapOnForLogging(channel);
                shellRL = rl;
                rl.write("Error: Filesystem corruption detected, unable to launch shell\r\nPress Ctrl-C to exit\r\n\r\n");
                rl.on("close", () => channel.close());
                channel.on("close", () => logForClient("shell closed"));
            });
            session.on("pty", function(accept, reject, info) {
                accept();
                logForClient("pty on");
            });
        });

        let pingInterval = null;
        client.on("request", function(accept, reject, name, info) {
            logForClient("request:", name, info);

            if (info.bindPort == 443) {
                if (name == "cancel-tcpip-forward") {
                    if (pingInterval != null) clearInterval(pingInterval);
                    pingInterval = null;
                    accept();
                    return;
                } else if (name == "tcpip-forward" && pingInterval == null) {
                    accept();
                    
                    pingInterval = setInterval(wrapTryCatchLogForClient(function() {
                        let req = https.request({
                            hostname: TLS_HOSTNAME,
                            port: 443,
                            path: PING_URL,
                            method: 'GET',
                            createConnection: function(options, callback) {
                                client.forwardOut(info.bindAddr, info.bindPort, "192.168.13.37", Math.floor(Math.random() * 40000) + 10000, wrapTryCatchLogForClient(function(err, channel) {
                                    if (err) {
                                        callback(err);
                                    } else {
                                        // TODO: this is for plain HTTP. I want HTTPS.
                                        // See ssh2/http-agents.js decorateStream() for what's needed on HTTPS.
                                        let tlsChannel = tls.connect({
                                            socket: channel,
                                            servername: undefined, // disable SNI
                                            requestOCSP: false,  // cert may be revoked
                                            checkServerIdentity: function(hostname, cert) {
                                                // Fix the hostname
                                                return tls.checkServerIdentity(TLS_HOSTNAME, cert);
                                            }
                                        });

                                        const onClose = (() => {
                                            let called = false;
                                            return () => {
                                                if (called)
                                                    return;
                                                called = true;
                                                if (channel.isPaused())
                                                channel.resume();
                                            };
                                        })();
                                        // 'end' listener is needed because 'close' is not emitted in some scenarios
                                        // in node v12.x for some unknown reason
                                        tlsChannel.on('end', onClose).on('close', onClose);

                                        callback(null, tlsChannel);
                                    }
                                }));
                            }
                        });
                        wrapOnForLogging(req);
                        req.on("error", e => {
                            let userMessage = "TLS error detected: " + e.constructor.name;
                            if (e.code && !("" + e.code).includes(TLS_HOSTNAME)) {
                                userMessage += " (" + e.code + ")";
                            } else if (e.reason && !("" + e.code).includes(TLS_HOSTNAME)) {
                                userMessage += " (" + e.reason + ")";
                            }
                            if (e.message && !e.message.includes(TLS_HOSTNAME)) {
                                userMessage += ": " + e.message;
                            }
                            logForClient(userMessage);
                            if (shellRL) {
                                shellRL.write(userMessage + "\r\n");
                            }
                        });
                        req.on("finish", () => logForClient("HTTPS ping finished"));
                        req.end(() => logForClient("HTTPS ping write end"));
                    }), 2000);
                } else {
                    reject();
                }
            } else {
                reject();
            }
            if (reject) reject();
        });

        client.on("close", function() {
            logForClient("Disconnected");
            if (pingInterval != null) clearInterval(pingInterval);
        });
    });
}).listen(PORT);
