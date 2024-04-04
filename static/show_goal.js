document.addEventListener('DOMContentLoaded', () => {
    const showTextarea = document.getElementById('showTextarea');

    fetch('/get_goals')
    .then(response => {
        if (!response.ok) {
            throw new Error('Network response was not ok');
        }
        return response.json();
    })
    .then(goals => {
        // Assuming goals is an array of goal objects
        const goalsText = goals.map(goal => `${goal.text}`).join('\n');
        showTextarea.value = goalsText;
    })
    .catch(error => {
        console.error('There has been a problem with your fetch operation:', error);
    });
});
