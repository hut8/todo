import React from 'react';
import TodoItem from './TodoItem';
import AddTodo from './AddTodo';
import Settings from './Settings';
import { useState, useEffect } from 'react';

const TodoList = (props) => {
    const [error, setError] = useState(null);
    const [isLoaded, setIsLoaded] = useState(false);
    const [items, setItems] = useState([]);
    const LIST_URL = new URL(Settings.apiBase);

    const loadItems = () => {
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
    };

    useEffect(loadItems, []);

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
      <>
        <AddTodo loadItems={loadItems} />
        <table className='table table-hover table-striped'>
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
                <TodoItem loadItems={loadItems} key={task.id} task={task} />
            ))}
          </tbody>
        </table>
      </>
    );
}

export default TodoList;
