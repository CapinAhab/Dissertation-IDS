
document.getElementById("createform").addEventListener("submit", function(event) {
    event.preventDefault(); // Prevent the default form submission

    var form = event.target;
    var formData = new FormData(form);

    document.getElementById("created").innerText = "Creating Model....";

    fetch(form.action, {
        method: 'POST',
        body: formData
    })

    .then(response => response.text())
    .then(data => {

        document.getElementById("created").innerText = "Model created";
    })
    .catch(error => {
        console.error('Error:', error);
    });
});
