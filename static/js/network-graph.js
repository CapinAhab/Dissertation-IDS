var ctx = document.getElementById('networkGraph').getContext('2d');
var networkGraph = new Chart(ctx, {
    type: 'line',
    data: {
	labels: [],
	datasets: [{
	    label: 'Network Traffic',
	    data: [],
	    fill: false,
	    borderColor: 'rgb(75, 192, 192)',
	    tension: 0.1
	}]
    },
    options: {
	scales: {
	    x: {
		type: 'linear',
		position: 'bottom'
	    },
	    y: {
		beginAtZero: true
	    }
	}
    }
})
