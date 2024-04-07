document.getElementById("train_button").addEventListener("click", function() {
    fetch("/trainmodel")
    .then(response => response.json())
    .then(data => {
	if (data.trained === true) {
	    document.getElementById("train_status").textContent = "Training Successful";
	} else {
	    // Handle the case if the response is not true
	    document.getElementById("train_status").textContent = "Training Failed";
	}
    })
    .catch(error => {
	console.error('Error:', error);
    });
});
