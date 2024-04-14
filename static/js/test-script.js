document.getElementById("test_button").addEventListener("click", function() {
    fetch("http://127.0.0.1:5000/test")
    .then(response => response.json())
    .then(data => {
        document.getElementById("accuracy").textContent = data.accuracy;
    })
    .catch(error => {
        console.error('Error:', error);
    });
});
