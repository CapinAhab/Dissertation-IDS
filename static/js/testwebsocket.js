// Shared worker script (shared-worker.js)
let socket;
onconnect = function (e) {
  const port = e.ports[0];
  port.onmessage = function (event) {
    if (event.data === 'open') {
	socket = new WebSocket('ws://echo');
	socket.addEventListener('open', (event) => {
	    // Sending data to the WebSocket
	    socket.send('Hello, WebSocket!');
	});
      // Handle WebSocket events here
    }
  };
};
