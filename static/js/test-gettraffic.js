
function single_get(){
    // Replace 'https://api.example.com/data' with your API endpoint
    const apiUrl = '/gettraffic';

    // Make a GET request using fetch
    fetch(apiUrl)
    .then(response => {
	// Check if the request was successful (status code 200-299)
	if (!response.ok) {
	    throw new Error(`HTTP error! Status: ${response.status}`);
	}

	// Parse the JSON response
	return response.json();
    })
    .then(data => {
	// Display the result in the console
	console.log(data);
    })
    .catch(error => {
	console.error('Error:', error);
    });
}

function infinate_get(){
    // Replace 'https://api.example.com/data' with your API endpoint
    const apiUrl = '/gettraffic';
    while (true){
	// Make a GET request using fetch
	fetch(apiUrl)
	.then(response => {
	    // Check if the request was successful (status code 200-299)
	    if (!response.ok) {
		throw new Error(`HTTP error! Status: ${response.status}`);
	    }

	    // Parse the JSON response
	    return response.json();
	})
	.then(data => {
	    // Display the result in the console
	    console.log(data);
	})
	.catch(error => {
	    console.error('Error:', error);
	});
    }
}
