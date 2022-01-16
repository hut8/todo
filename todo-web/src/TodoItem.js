import React from 'react';
import PropTypes from 'prop-types';
import Settings from './Settings';
import axios from 'axios';
import moment from 'moment';

function TodoItem(props) {
    let task = props.task;
    let loadItems = props.loadItems;

    const toggleCompleted = (task) => {
        task.completed = !task.completed;
        let url = new URL(Settings.apiBase);
        url.pathname = `${url.pathname}/${task.id}`;
        let response = axios.put(url, task);
        response.then((v) => {
            console.log("updated: ", task.id, v.statusText);
            loadItems();
        });
    };

    const deleteItem = (task) => {
        let url = new URL(Settings.apiBase);
        url.pathname = `${url.pathname}/${task.id}`;
        console.log(`deleteItem: ${task.id}`);
        let response = axios.delete(url);
        response.then((v) => {
            console.log("delete", task.id, v.statusText);
            loadItems();
        });
    };

    const dt = moment(task.created_at);

    return (
        <tr className="todo-item">
            <td>{task.description}</td>
            <td>{dt.format('LLLL')}</td>
            <td>
                <input type="checkbox"
                    onChange={() => toggleCompleted(task)}
                    checked={task.completed} />
            </td>
            <td style={{cursor: 'pointer'}} onClick={() => deleteItem(task)}>
                ❌
            </td>
        </tr>
    )
}

TodoItem.propTypes = {
    task: PropTypes.object.isRequired,
    loadItems: PropTypes.func.isRequired,
};

export default TodoItem;

