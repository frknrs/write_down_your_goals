document.addEventListener('DOMContentLoaded', () => {
  const deleteButton = document.getElementById('deleteAll');

  deleteButton.addEventListener('click', () => {
    if (confirm('Are you sure you want to delete all goals? This action cannot be undone.')) {
      fetch('/clean_table', { method: 'GET' })
        .then(response => {
          if (response.ok) {
            return response.json();
          }
          throw new Error('Network response was not ok.');
        })
        .then(data => {
          alert(data); // Or any other indication of success
          // Optionally, redirect or update the UI to reflect the deletion
        })
        .catch(error => console.error('There has been a problem with your fetch operation:', error));
    }
  });
});

