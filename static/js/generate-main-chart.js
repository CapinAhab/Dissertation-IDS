// Creates a WebSocket connection
const socket = new WebSocket('ws://127.0.0.1:8000/gettraffic');

// Event handler for when a message is received from the server
socket.addEventListener('message', (event) => {
    const message = event.data;
    const jsonObject = JSON.parse(message);

    // Now, jsonObject is a JavaScript object
    console.log(jsonObject);
    console.log('Message from server:', message);

    if (jsonObject.malicious) {
	console.log("Malicious")
        // Create a <p> element
        const paragraph = document.createElement('p');
        // Set the text of the <p> element
        paragraph.textContent = `Potentially malicious packet from: ${jsonObject.sender}`;
        
        // Get the <div> element with id "malicious-div"
        const maliciousDiv = document.getElementById('malicious-div');
        // Append the <p> element to the <div>
        maliciousDiv.appendChild(paragraph);
    }
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
