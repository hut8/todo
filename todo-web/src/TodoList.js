import React from 'react';
import TodoItem from './TodoItem';
import { useState, useEffect } from 'react';

const TodoList = (props) => {
    const [error, setError] = useState(null);
    const [isLoaded, setIsLoaded] = useState(false);
    const [items, setItems] = useState([]);
    const LIST_URL = "http://127.0.0.1:8000/tasks";

    useEffect(() => {
        fetch(LIST_URL)
          .then(res => res.json())
          .then(
            (result) => {
              setIsLoaded(true);
              setItems(result);
            },
            // Note: it's important to handle errors here
            // instead of a catch() block so that we don't swallow
            // exceptions from actual bugs in components.
            (error) => {
              setIsLoaded(true);
              setError(error);
            }
          )
      }, []);

    if (error) {
        return (
            <b>Error: {error.message}</b>
        );
    }

    if (!isLoaded) {
      return (
        <b>Loading...</b>
      );
    }

    return (
        <table>
          <thead>
            <tr>
              <th>Description</th>
              <th>📅</th>
              <th>✔</th>
              <th>❌</th>
            </tr>
          </thead>
          <tbody>
            {items.map((task) => (
                <TodoItem key={task.id} task={task} />
            ))}
          </tbody>
        </table>
    );
}

export default TodoList;
