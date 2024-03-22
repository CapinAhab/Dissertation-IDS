// Creates a WebSocket connection
const socket = new WebSocket('ws://127.0.0.1:8000/gettraffic');

// Event handler for when a message is received from the server
socket.addEventListener('message', (event) => {
    const message = event.data;
    const jsonObject = JSON.parse(message);

    // Now, jsonObject is a JavaScript object
    console.log(jsonObject);
    console.log('Message from server:', message);

    //Update graph

});

// Event handler for when an error occurs
socket.addEventListener('error', (event) => {
    console.error('WebSocket error:', event);
});

// Event handler for when the connection is closed
socket.addEventListener('close', (event) => {
    console.log('WebSocket connection closed:', event);
});
