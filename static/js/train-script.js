document.getElementById("trainform").addEventListener("submit", function(event) {
    event.preventDefault(); // Prevent the default form submission

    var form = event.target;
    var formData = new FormData(form);

    document.getElementById("trained").innerText = "Training....";
    fetch(form.action, {
        method: 'POST',
        body: formData
    })

    .then(response => response.text())
    .then(data => {
        document.getElementById("trained").innerText = "Trained";
    })
    .catch(error => {
        console.error('Error:', error);
    });
});
