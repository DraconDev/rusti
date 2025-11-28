// Initialize Chart.js chart
const ctx = document.getElementById("myChart").getContext("2d");
new Chart(ctx, {
    type: "bar",
    data: {
        labels: ["Jan", "Feb", "Mar", "Apr", "May", "Jun"],
        datasets: [
            {
                label: "Sales",
                data: [12, 19, 3, 5, 2, 3],
                backgroundColor: "rgba(52, 152, 219, 0.6)",
                borderColor: "rgb(52, 152, 219)",
                borderWidth: 1,
            },
        ],
    },
    options: {
        scales: {
            y: {
                beginAtZero: true,
            },
        },
    },
});
