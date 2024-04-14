document.getElementById("test_button").addEventListener("click", function() {
    fetch("/testmodel")
    .then(response => response.json())
    .then(data => {
	document.getElementById("accuracy").textContent = data.accuracy;
    })
    .catch(error => {
	console.error('Error:', error);
    });
});
