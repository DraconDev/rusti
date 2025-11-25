// Hello World - Simple interactivity
document.addEventListener('DOMContentLoaded', () => {
    const button = document.querySelector('button.primary');
    
    if (button) {
        button.addEventListener('click', () => {
            alert('Hello from external JavaScript! 🎉');
        });
    }
});
