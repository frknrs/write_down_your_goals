const goalForm = document.getElementById('goalForm');
const goalTextarea = document.getElementById('goalTextarea');

goalForm.addEventListener('submit', async (event) => {
  event.preventDefault(); // Prevent default form submission

  const goalText = goalTextarea.value.trim();

  if (!goalText) {
    alert('Please enter your goals for today!');
    return;
  }

  const response = await fetch('/add_goal', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ text: goalText }),
  });

  if (!response.ok) {
    console.error('Error adding goal:', response.statusText);
    alert('Error adding goal!');
    return;
  }

  const data = await response.json();
  console.log('Goal added:', data);

  // Clear the goal text input after successful submission
  goalTextarea.value = '';

  alert(data.message); // Display success message from response
});

// Accessibility improvement (optional)
const goalTextareaLabel = document.createElement('label');
goalTextareaLabel.setAttribute('for', 'goalTextarea');
goalForm.insertBefore(goalTextareaLabel, goalTextarea);
