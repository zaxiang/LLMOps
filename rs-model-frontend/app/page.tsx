'use client';
import React, { useState } from 'react';
import './page.css'; // Importing the CSS file

const Page = () => {
  const [input, setInput] = useState(''); // State to store user input
  const [response, setResponse] = useState(''); // State to store the server response

  const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault(); // Prevents the default form submit behavior (page reload)
    try {
      const response = await fetch('http://localhost:8080/', { // Replace '/api/endpoint' with your actual API endpoint
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ message: input }), // Send the user input as JSON
      });
      const data = await response.json(); // Assume the server responds with JSON
      console.log('Data:', data.response);
      setResponse(data.response); // Update the response state with the message from the server
    } catch (error) {
      console.error('Error:', error);
      setResponse('Failed to get response from server'); // Handle error situation
    }
  };

  return (
    <div className="container">
      <header className="header">
        <h1>Model Serving with Rust</h1>
      </header>
      <div className="subtitle">
        <p>Enter your input below to generate the rest of the text.</p>
      </div>
      <div className="input-container">
        <form className="form" onSubmit={handleSubmit}>
          <input
            className="input"
            type="text"
            placeholder="Enter your input"
            value={input}
            onChange={(e) => setInput(e.target.value)}
            aria-label="Input field for user data"
          />
          <button className="button" type="submit" aria-label="Submit user input">Submit</button>
        </form>  
      </div>
      {response && <div className="response">{response}</div>} {/* Display the response from the server */}
    </div>
  );
}

export default Page;
