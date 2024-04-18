document.getElementById("test_button").addEventListener("submit", function(event) {
    event.preventDefault(); // Prevent the default form submission

    var form = event.target;
    var formData = new FormData(form);

    document.getElementById("accuracy").innerText = "Testing....";
    fetch(form.action, {
        method: 'POST',
        body: formData
    })

    .then(response => response.json())
    .then(data => {
	document.getElementById("accuracy").textContent = data.accuracy;
    })
    .catch(error => {
        console.error('Error:', error);
    });
});
