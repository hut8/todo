import React from 'react';
import PropTypes from 'prop-types';
import Settings from './Settings';
import axios from 'axios';

function TodoItem(props) {
    let task = props.task;

    const completedChange = (e) => {
        console.log(`completedChange: ${e.target.value}`);
    };

    const deleteItem = (task) => {
        let url = new URL(Settings.apiBase);
        url.pathname = `${url.pathname}/${task.id}`;
        console.log(`deleteItem: ${task.id}`);
        let response = axios.delete(url);
        response.then((v) => console.log("delete",task.id,v.statusText));
    };

    return (
        <tr className="todo-item">
            <td>{task.description}</td>
            <td>{task.created_at}</td>
            <td><input type="checkbox"
                onChange={(e) => completedChange(e)}
                checked={task.completed} /></td>
            <td style={{cursor: 'pointer'}} onClick={() => deleteItem(task)}>
                ❌
            </td>
        </tr>
    )
}

TodoItem.propTypes = {
    task: PropTypes.object.isRequired,
};

export default TodoItem;

