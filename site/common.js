function send_data(o) {
  return fetch('/site', {
    method: 'POST',
    headers: {
      'Accept': 'application/json',
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(o)
  });
}
