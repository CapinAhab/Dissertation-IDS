const socket = new WebSocket("ws://127.0.0.1:8000/gettraffic");

var totalPackets = 0;
var maliciousPackets = 0;

var ctx = document.getElementById('networkGraph').getContext('2d');
var networkGraph = new Chart(ctx, {
    type: 'line', // or 'bar' if you prefer a bar chart
    data: {
        labels: Array.from({length: 61}, (_, i) => i), // generates an array [0, 1, 2, ..., 60]
        datasets: [{
            label: '# TCP packets on the network',
            data: Array(61).fill(0), // Initialize with zeros
            backgroundColor: 'rgba(255, 99, 132, 0.2)',
            borderColor: 'rgba(255, 99, 132, 1)',
            borderWidth: 1
        }]
    },
    options: {
        scales: {
            y: {
                beginAtZero: true
            }
        }
    }
});

setInterval(() => {
    // Add zero pings if no message is received
    networkGraph.data.datasets[0].data.push(0);
    networkGraph.data.datasets[0].data.shift();

    // Update the chart
    networkGraph.update();
}, 1000);

// WebSocket onmessage event
socket.addEventListener('message', (event) => {
    // Now, jsonObject is a JavaScript object
    console.log('Message from server:', event.data);

    const jsonObject = JSON.parse(event.data);
    if (jsonObject.malicious) {
	maliciousPackets++;
	console.log("Malicious")
        // Create a <p> element
        const paragraph = document.createElement('p');
        // Set the text of the <p> element
        paragraph.textContent = `Potentially malicious packet from: ${jsonObject.source}`;
        
        // Get the <div> element with id "malicious-div"
        const maliciousDiv = document.getElementById('malicious-div');
        // Append the <p> element to the <div>
        maliciousDiv.appendChild(paragraph);
    }

    totalPackets++;
    
    // Update total pings count in <p> element
    const totalPacketsParagraph = document.getElementById('total-pings');
    totalPacketsParagraph.textContent = `Total pings: ${totalPackets}`;

    // Update malicious pings count in <p> element
    const maliciousPacketsParagraph = document.getElementById('malicious-pings');
    maliciousPacketsParagraph.textContent = `Malicious pings: ${maliciousPings}`;
    
    // Update chart data
    networkGraph.data.datasets[0].data.push(1);
    networkGraph.data.datasets[0].data.shift();
    
    // Update the chart
    networkGraph.update();
});

// Event handler for when an error occurs
socket.addEventListener('error', (event) => {
    console.error('WebSocket error:', event);
});

// Event handler for when the connection is closed
socket.addEventListener('close', (event) => {
    console.log('WebSocket connection closed:', event);
});
