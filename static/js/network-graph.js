const socket = new WebSocket("ws://127.0.0.1:8000/gettraffic");

var ctx = document.getElementById('networkGraph').getContext('2d');
var networkGraph = new Chart(ctx, {
    type: 'line', // or 'bar' if you prefer a bar chart
    data: {
        labels: Array.from({length: 61}, (_, i) => i), // generates an array [0, 1, 2, ..., 60]
        datasets: [{
            label: '# of Pings',
            data: [/* Your data array goes here */],
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
    // Update chart data
    networkGraph.data.datasets[0].data.push(1);
    networkGraph.data.datasets[0].data.shift();
    
    // Update the chart
    networkGraph.update();
    // Now, jsonObject is a JavaScript object
    console.log(jsonObject);
    console.log('Message from server:', message);

    //Update graph


    // Increment ping count
    pingCount++;

    // Update chart
    pingChart.data.labels.push(new Date());
    pingChart.data.datasets[0].data.push(pingCount);
    pingChart.update();

});

// Event handler for when an error occurs
socket.addEventListener('error', (event) => {
    console.error('WebSocket error:', event);
});

// Event handler for when the connection is closed
socket.addEventListener('close', (event) => {
    console.log('WebSocket connection closed:', event);
});

